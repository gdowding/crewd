import React, { useState, useEffect, useRef, useCallback } from 'react';
import { Users, ExternalLink, CheckCircle, XCircle, HelpCircle, Filter, LogIn, LogOut, User, Edit3, Loader2 } from 'lucide-react';
import { SCHEDULE_DATA } from './data/schedule.js';
import AuthModal from './components/AuthModal.jsx';
import ProfileModal from './components/ProfileModal.jsx';
import { supabase } from './lib/supabase.js';

export default function App() {
  const [events] = useState(SCHEDULE_DATA);
  const [rsvpState, setRsvpState] = useState({});
  const [attendeesByEvent, setAttendeesByEvent] = useState({});
  const [savingEventId, setSavingEventId] = useState(null);
  const [showAllEvents, setShowAllEvents] = useState(false);
  const [isAuthOpen, setIsAuthOpen] = useState(false);
  const [isProfileOpen, setIsProfileOpen] = useState(false);
  const [currentUser, setCurrentUser] = useState(null);

  const isFetchingRef = useRef(false);

  const getDisplayName = (user) => {
    return user?.user_metadata?.username || user?.email?.split('@')[0] || 'Crew';
  };

  // Fetch RSVPs from database without setting triggering dependencies
  const loadRsvps = useCallback(async (userId) => {
    if (isFetchingRef.current) return;
    isFetchingRef.current = true;

    try {
      const { data, error } = await supabase.from('rsvps').select('*');
      if (!error && data) {
        const userMap = {};
        const attendeesMap = {};

        data.forEach(item => {
          if (userId && item.user_id === userId) {
            userMap[item.event_id] = item.status;
          }

          if (item.status === 'attending') {
            if (!attendeesMap[item.event_id]) {
              attendeesMap[item.event_id] = [];
            }
            const name = item.username || item.user_email?.split('@')[0] || 'Crew';
            if (!attendeesMap[item.event_id].includes(name)) {
              attendeesMap[item.event_id].push(name);
            }
          }
        });

        setRsvpState(userMap);
        setAttendeesByEvent(attendeesMap);
      }
    } catch (e) {
      console.error('Failed to load RSVPs', e);
    } finally {
      isFetchingRef.current = false;
    }
  }, []);

  // Run Auth check ONCE on initial mount
  useEffect(() => {
    let isMounted = true;

    async function initAuth() {
      const { data: { session } } = await supabase.auth.getSession();
      if (isMounted) {
        const user = session?.user ?? null;
        setCurrentUser(user);
        loadRsvps(user?.id);
      }
    }

    initAuth();

    const { data: { subscription } } = supabase.auth.onAuthStateChange((_event, session) => {
      if (isMounted) {
        const user = session?.user ?? null;
        setCurrentUser(user);
        loadRsvps(user?.id);
      }
    });

    return () => {
      isMounted = false;
      subscription.unsubscribe();
    };
  }, []); // Strictly empty dependency array to prevent loops

  const handleRsvp = async (eventId, status) => {
    if (!currentUser) {
      setIsAuthOpen(true);
      return;
    }

    const activeUsername = getDisplayName(currentUser);
    setSavingEventId(eventId);

    // 1. Local state update
    setRsvpState(prev => ({ ...prev, [eventId]: status }));
    setAttendeesByEvent(prev => {
      const currentList = prev[eventId] || [];
      if (status === 'attending') {
        return {
          ...prev,
          [eventId]: Array.from(new Set([...currentList, activeUsername]))
        };
      } else {
        return {
          ...prev,
          [eventId]: currentList.filter(name => name !== activeUsername)
        };
      }
    });

    // 2. Database upsert
    try {
      await supabase
        .from('rsvps')
        .upsert({
          user_id: currentUser.id,
          user_email: currentUser.email,
          username: activeUsername,
          event_id: eventId,
          status: status
        }, { onConflict: 'user_id,event_id' });
    } catch (err) {
      console.error('RSVP save error:', err);
    } finally {
      setSavingEventId(null);
    }
  };

  const handleLogout = async () => {
    await supabase.auth.signOut();
    setCurrentUser(null);
    setRsvpState({});
  };

  const today = new Date();
  const fourWeeksLater = new Date(today);
  fourWeeksLater.setDate(today.getDate() + 28);

  const displayedEvents = showAllEvents
    ? events
    : events.filter(evt => {
        const eventDate = new Date(evt.startDate);
        return eventDate >= today && eventDate <= fourWeeksLater;
      });

  return (
    <div className="min-h-screen bg-slate-900 text-slate-100 font-sans">
      <header className="bg-slate-800 border-b border-slate-700 px-6 py-4 flex justify-between items-center sticky top-0 z-10">
        <div className="flex items-center space-x-3">
          <h1 className="text-xl font-bold tracking-wide text-white">IRIE RACING 2026</h1>
          <span className="bg-orange-500/20 text-orange-400 text-xs px-2.5 py-0.5 rounded-full border border-orange-500/30 font-semibold">
            {displayedEvents.length} {showAllEvents ? 'Total Events' : 'Next 4 Weeks'}
          </span>
        </div>

        <div>
          {currentUser ? (
            <div className="flex items-center space-x-3">
              <button
                onClick={() => setIsProfileOpen(true)}
                className="text-xs text-slate-300 flex items-center gap-1.5 bg-slate-700/60 hover:bg-slate-700 px-3 py-1.5 rounded-full border border-slate-600 transition"
              >
                <User className="w-3.5 h-3.5 text-orange-400" />
                <span>{getDisplayName(currentUser)}</span>
                <Edit3 className="w-3 h-3 text-slate-400 ml-1" />
              </button>
              <button
                onClick={handleLogout}
                className="flex items-center space-x-1 bg-slate-700 hover:bg-slate-600 text-xs font-semibold px-3 py-1.5 rounded transition text-slate-200"
              >
                <LogOut className="w-3.5 h-3.5" />
                <span>Logout</span>
              </button>
            </div>
          ) : (
            <button
              onClick={() => setIsAuthOpen(true)}
              className="flex items-center space-x-1.5 bg-orange-500 hover:bg-orange-600 text-white text-xs font-semibold px-3.5 py-1.5 rounded transition"
            >
              <LogIn className="w-3.5 h-3.5" />
              <span>Crew Login</span>
            </button>
          )}
        </div>
      </header>

      <main className="p-6 max-w-4xl mx-auto space-y-6">
        {/* Filter Toggle Toolbar */}
        <div className="flex justify-between items-center pt-2">
          <div>
            <h2 className="text-lg font-bold text-white">
              {showAllEvents ? 'All 2026 Regattas' : 'Upcoming Races (Next 4 Weeks)'}
            </h2>
            <p className="text-xs text-slate-400">
              {showAllEvents ? 'Showing complete season' : `Filtered: ${today.toISOString().split('T')[0]} to ${fourWeeksLater.toISOString().split('T')[0]}`}
            </p>
          </div>

          <button
            onClick={() => setShowAllEvents(!showAllEvents)}
            className="flex items-center space-x-2 bg-slate-800 hover:bg-slate-700 border border-slate-700 text-xs font-semibold px-3 py-2 rounded-md transition text-slate-200"
          >
            <Filter className="w-3.5 h-3.5 text-orange-400" />
            <span>{showAllEvents ? 'Show Next 4 Weeks Only' : 'Show Full Season (54 Races)'}</span>
          </button>
        </div>

        <div className="flex flex-col space-y-4">
          {displayedEvents.map((evt) => {
            const currentStatus = rsvpState[evt.id] || 'pending';
            const dbAttendees = attendeesByEvent[evt.id] || [];
            const combinedCrew = Array.from(new Set([...(evt.crew || []), ...dbAttendees]));
            const isSaving = savingEventId === evt.id;

            return (
              <div key={evt.id} className="bg-slate-800 p-5 rounded-lg border border-slate-700 hover:border-slate-600 transition flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div className="flex-1 space-y-1">
                  <div className="flex items-center space-x-2">
                    <span className="text-xs uppercase font-bold tracking-wider text-orange-400">
                      {evt.series}
                    </span>
                    <span className="text-[10px] bg-slate-700 text-slate-300 px-2 py-0.5 rounded uppercase font-semibold">
                      {evt.sponsor}
                    </span>
                  </div>

                  <h3 className="text-lg font-semibold text-white">{evt.race}</h3>

                  <p className="text-xs text-slate-400">
                    📅 {evt.startDate} {evt.endDate !== evt.startDate ? `to ${evt.endDate}` : ''}
                  </p>

                  {evt.notes && (
                    <p className="text-xs text-amber-400 bg-amber-500/10 px-2.5 py-1 rounded border border-amber-500/20 inline-block mt-1">
                      ⚠️ {evt.notes}
                    </p>
                  )}
                </div>


		  {/* Middle Block: Assigned Crew (Protected) */}
		  <div className="md:w-1/3">
		      <p className="text-xs font-semibold text-slate-400 mb-1.5 flex items-center gap-1">
			  <Users className="w-3.5 h-3.5" /> Assigned Crew:
		      </p>

		      {currentUser ? (
			  /* Logged In: Show Live Combined Roster */
			  combinedCrew.length > 0 ? (
			      <div className="flex flex-wrap gap-1">
				  {combinedCrew.map((member, idx) => {
				      const isDbUser = dbAttendees.includes(member);
				      return (
					  <span
					      key={idx}
					      className={`text-[11px] px-2 py-0.5 rounded ${
                isDbUser
                  ? 'bg-emerald-950/80 text-emerald-300 border border-emerald-700/50 font-medium'
                  : 'bg-slate-700/80 text-slate-200'
              }`}
					  >
					      {member}
					  </span>
				      );
				  })}
			      </div>
			  ) : (
			      <span className="text-xs text-slate-500">No crew assigned yet</span>
			  )
		      ) : (
			  /* Logged Out: Privacy Protection Message */
			  <button
			      onClick={() => setIsAuthOpen(true)}
			      className="text-xs text-slate-500 hover:text-orange-400 border border-dashed border-slate-700 hover:border-orange-500/50 rounded px-2.5 py-1.5 transition text-left block w-full"
			  >
			      🔒 Log in to view crew roster ({combinedCrew.length})
			  </button>
		      )}
		  </div>


                <div className="flex md:flex-col items-center md:items-end justify-between md:justify-center border-t md:border-t-0 md:border-l border-slate-700/60 pt-3 md:pt-0 md:pl-5 gap-3">
                  <div className="flex items-center space-x-1.5 relative">
                    {isSaving && (
                      <Loader2 className="w-3.5 h-3.5 text-orange-400 animate-spin absolute -left-5" />
                    )}
                    <button
                      onClick={() => handleRsvp(evt.id, 'attending')}
                      disabled={isSaving}
                      className={`p-2 rounded transition ${
                        currentStatus === 'attending' ? 'bg-emerald-600 text-white' : 'bg-slate-700 text-slate-400 hover:text-white'
                      }`}
                      title="Attending"
                    >
                      <CheckCircle className="w-4 h-4" />
                    </button>
                    <button
                      onClick={() => handleRsvp(evt.id, 'tentative')}
                      disabled={isSaving}
                      className={`p-2 rounded transition ${
                        currentStatus === 'tentative' ? 'bg-amber-600 text-white' : 'bg-slate-700 text-slate-400 hover:text-white'
                      }`}
                      title="Tentative"
                    >
                      <HelpCircle className="w-4 h-4" />
                    </button>
                    <button
                      onClick={() => handleRsvp(evt.id, 'declined')}
                      disabled={isSaving}
                      className={`p-2 rounded transition ${
                        currentStatus === 'declined' ? 'bg-rose-600 text-white' : 'bg-slate-700 text-slate-400 hover:text-white'
                      }`}
                      title="Decline"
                    >
                      <XCircle className="w-4 h-4" />
                    </button>
                  </div>

                  {evt.eventUrl ? (
                    <a
                      href={evt.eventUrl}
                      target="_blank"
                      rel="noreferrer"
                      className="text-xs text-sky-400 hover:underline flex items-center gap-1"
                    >
                      Event Page <ExternalLink className="w-3 h-3" />
                    </a>
                  ) : (
                    <span className="text-xs text-slate-500">No URL</span>
                  )}
                </div>

              </div>
            );
          })}
        </div>
      </main>

      <AuthModal
        isOpen={isAuthOpen}
        onClose={() => setIsAuthOpen(false)}
        onSessionChange={(user) => {
          setCurrentUser(user);
          loadRsvps(user?.id);
        }}
      />

      <ProfileModal
        isOpen={isProfileOpen}
        onClose={() => setIsProfileOpen(false)}
        currentUser={currentUser}
        onProfileUpdated={(updatedUser) => {
          setCurrentUser(updatedUser);
          loadRsvps(updatedUser?.id);
        }}
      />
    </div>
  );
}
