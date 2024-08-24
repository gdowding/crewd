import React, { useState } from 'react';
import { supabase } from '../lib/supabase';
import { LogIn, X, User } from 'lucide-react';

export default function AuthModal({ isOpen, onClose, onSessionChange }) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [username, setUsername] = useState('');
  const [isSignUp, setIsSignUp] = useState(false);
  const [error, setError] = useState(null);

  if (!isOpen) return null;

  const handleAuth = async (e) => {
    e.preventDefault();
    setError(null);

    let result;
    if (isSignUp) {
      // Store username in user_metadata during registration
      result = await supabase.auth.signUp({
        email,
        password,
        options: {
          data: { username: username.trim() }
        }
      });
    } else {
      result = await supabase.auth.signInWithPassword({ email, password });
    }

    if (result.error) {
      setError(result.error.message);
    } else {
      if (result.data.session) {
        onSessionChange(result.data.session.user);
      }
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4">
      <div className="bg-slate-800 border border-slate-700 rounded-lg p-6 max-w-md w-full relative text-slate-100">
        <button onClick={onClose} className="absolute top-4 right-4 text-slate-400 hover:text-white">
          <X className="w-5 h-5" />
        </button>

        <h2 className="text-xl font-bold mb-1 text-white flex items-center gap-2">
          <LogIn className="w-5 h-5 text-orange-400" />
          {isSignUp ? 'Create Crew Account' : 'Crew Login'}
        </h2>
        <p className="text-xs text-slate-400 mb-4">
          Sign in to manage your RSVPs and race availability for Irie.
        </p>

        {error && (
          <div className="mb-4 text-xs text-rose-400 bg-rose-500/10 p-2.5 rounded border border-rose-500/20">
            {error}
          </div>
        )}

        <form onSubmit={handleAuth} className="space-y-4">
          {/* Username Field (Sign Up Only) */}
          {isSignUp && (
            <div>
              <label className="block text-xs font-semibold text-slate-300 mb-1">
                Crew Username / Display Name
              </label>
              <div className="relative">
                <input
                  type="text"
                  required
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 pl-9 text-sm text-white focus:outline-none focus:border-orange-500"
                  placeholder=""
                />
                <User className="w-4 h-4 text-slate-500 absolute left-2.5 top-2.5" />
              </div>
              <p className="text-[11px] text-slate-400 mt-1">
                Must match your name as listed in the assigned crew list.
              </p>
            </div>
          )}

          <div>
            <label className="block text-xs font-semibold text-slate-300 mb-1">Email</label>
            <input
              type="email"
              required
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white focus:outline-none focus:border-orange-500"
              placeholder="crew@sailirie.com"
            />
          </div>

          <div>
            <label className="block text-xs font-semibold text-slate-300 mb-1">Password</label>
            <input
              type="password"
              required
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full bg-slate-900 border border-slate-700 rounded px-3 py-2 text-sm text-white focus:outline-none focus:border-orange-500"
              placeholder="••••••••"
            />
          </div>

          <button
            type="submit"
            className="w-full bg-orange-500 hover:bg-orange-600 text-white font-semibold py-2 rounded text-sm transition"
          >
            {isSignUp ? 'Sign Up' : 'Log In'}
          </button>
        </form>

        <div className="mt-4 text-center">
          <button
            onClick={() => setIsSignUp(!isSignUp)}
            className="text-xs text-sky-400 hover:underline"
          >
            {isSignUp ? 'Already have an account? Log in' : "Need an account? Sign up"}
          </button>
        </div>
      </div>
    </div>
  );
}
