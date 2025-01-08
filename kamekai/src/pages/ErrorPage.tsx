import React from 'react';
import { Button } from '@/components/ui/button';
import { Home } from 'lucide-react';

interface ErrorPageProps {
  error: string;
  onBack: () => void;
}

export const ErrorPage: React.FC<ErrorPageProps> = ({ error, onBack }) => {
  return (
    <div className="min-h-screen bg-[#1E1E1E] text-white flex flex-col items-center justify-center p-4">
      <div className="flex flex-col items-center max-w-md text-center space-y-6">
        <div className="relative">
          <div className="text-gray-500">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="160"
              height="160"
              viewBox="0 0 40 40"
              fill="none"
              stroke="currentColor"
              strokeWidth="0.8"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              {/* Robot head */}
              <rect x="8" y="6" width="24" height="28" rx="2" />

              {/* Antenna */}
              <line x1="20" y1="2" x2="20" y2="6" />
              <circle cx="20" cy="2" r="1" />

              {/* Eyes */}
              <g className="animate-pulse">
                <rect x="13" y="14" width="4" height="4" />
                <rect x="23" y="14" width="4" height="4" />
              </g>

              {/* Glitchy mouth */}
              <path d="M14 24 l2 2 l2 -2 l2 2 l2 -2 l2 2 l2 -2" />
            </svg>
          </div>
        </div>
        <div className="space-y-2">
          <h3 className="text-2xl font-semibold text-white">Oops! Something went wrong</h3>
          <p className="text-gray-400">{error}</p>
          <p className="text-sm text-gray-500">Please try again in a few moments</p>
        </div>

        <Button
          onClick={onBack}
          variant="outline"
          className="mt-6 bg-transparent border-gray-700 text-gray-300 hover:bg-gray-800 hover:text-white"
        >
          <Home className="h-4 w-4 mr-2" />
          Return Home
        </Button>
      </div>
    </div>
  );
};
