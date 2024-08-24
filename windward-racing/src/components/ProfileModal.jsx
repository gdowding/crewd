import React, { useState } from 'react';
import { supabase } from '../lib/supabase';
import { User, X } from 'lucide-react';

export default function ProfileModal({ isOpen, onClose, currentUser, onProfileUpdated }) {
  const currentUsername = currentUser?.user_metadata?.username || currentUser?.email?.split('@')[0] || '';
  const [newUsername, setNewUsername] = useState(currentUsername);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState(null);

  if (!isOpen) return null;

  const handleUpdateProfile = async (e) => {
    e.preventDefault();
    setLoading(true);
    setMessage(null);

    const trimmed = newUsername.trim();
    if (!trimmed) {
      setMessage({ type: 'error', text: 'Display name cannot be empty.' });
      setLoading(false);
      return;
    }

    // 1. Update user metadata in Supabase Auth
    const { data, error } = await supabase.auth.updateUser({
      data: { username: trimmed }
    });

    if (error) {
      setMessage({ type: 'error', text: error.message });
    } else {
      // 2. Update username across existing RSVPs in the database
      if (currentUser?.id) {
        await supabase
          .from('rsvps')
          .update({ username: trimmed })
          .eq('user_id', currentUser.id);
      }

      setMessage({ type: 'success', text: 'Display name updated successfully!' });

      if (onProfileUpdated && data.user) {
        onProfileUpdated(data.user);
      }

      setTimeout(() => {
        onClose();
        setMessage(null);
      }, 1000);
    }
    setLoading(false);
  };

  return (
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4">
      <div className="bg-slate-800 border border-slate-700 rounded-lg p-6 max-w-md w-full relative text-slate-100">
        <button onClick={onClose} className="absolute top-4 right-4 text-slate-400 hover:text-white">
          <X className="w-5 h-5" />
        </button>

        <h2 className="text-xl font-bold mb-1 text-white flex items-center gap-2">
          <User className="w-5 h-5 text-orange-400" />
          Edit Crew Display Name
        </h2>
        <p className="text-xs text-slate-400 mb-4">
          Update the name displayed in your header and saved with your race RSVPs.
        </p>

        {message && (
          <div className={`mb-4 text-xs p-2.5 rounded border ${
            message.type === 'error'
              ? 'text-rose-400 bg-rose-500/10 border-rose-500/20'
              : 'text-emerald-400 bg-emerald-500/10 border-emerald-500/20'
          }`}>
            {message.text}
          </div>
        )}

        <form onSubmit={handleUpdateProfile} className="space-y-4">
          <div>
            <label className="block text-xs font-semibold text-slate-300 mb-1">
              Display Name / Username
            </label>
            <input
              type="text"
              required
              value={newUsername}
              onChange={(e) => setNewUsername(e.target.value)}
              className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white focus:outline-none focus:border-orange-500"
              placeholder="e.g., George, Davor, Jeff P"
            />
            <p className="text-[11px] text-slate-400 mt-1">
              Match your name as listed on the assigned crew roster.
            </p>
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full bg-orange-500 hover:bg-orange-600 text-white font-semibold py-2 rounded text-sm transition flex items-center justify-center gap-2"
          >
            {loading ? 'Saving...' : 'Update Display Name'}
          </button>
        </form>
      </div>
    </div>
  );
}
