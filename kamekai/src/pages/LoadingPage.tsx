import React, { useEffect, useState } from 'react';
import { Loader2 } from 'lucide-react';

const loadingMessages = [
  'Still working on it...',
  'Almost there...',
  'Processing languages...',
  'Analyzing grammar...',
  'Generating examples...',
  'Finalizing translations...',
  'This is taking longer than expected...',
  'Thanks for your patience...',
];

interface LoadingPageProps {
  isLoading: boolean;
}

export const LoadingPage: React.FC<LoadingPageProps> = ({ isLoading }) => {
  const [progress, setProgress] = useState(0);
  const [messageIndex, setMessageIndex] = useState(0);

  useEffect(() => {
    if (!isLoading) {
      // Quickly complete the progress when request finishes.
      setProgress(100);
      return;
    }

    // Start with faster progress.
    const progressInterval = setInterval(() => {
      setProgress(prev => {
        // Slow down as we get higher.
        const increment = Math.max(0.5, (100 - prev) * 0.1);
        const newProgress = prev + increment;
        return newProgress >= 99 ? 99 : newProgress;
      });
    }, 100);

    // Rotate messages
    const messageInterval = setInterval(() => {
      setMessageIndex(prev => (prev + 1) % loadingMessages.length);
    }, 3000);

    return () => {
      clearInterval(progressInterval);
      clearInterval(messageInterval);
    };
  }, [isLoading]);

  return (
    <div className="min-h-screen bg-[#1E1E1E] text-white flex flex-col items-center justify-center">
      <div className="relative flex flex-col items-center">
        <Loader2 className="w-16 h-16 text-blue-500 animate-spin mb-8" />
        <div className="space-y-2 text-center">
          <h3 className="text-2xl font-semibold">Translating your text</h3>
          <p className="text-gray-400 h-6 transition-all duration-300">
            {loadingMessages[messageIndex]}
          </p>
          <div className="mt-8 w-48">
            <div className="h-2 bg-gray-800 rounded-full overflow-hidden">
              <div
                className="h-full bg-blue-500 transition-all duration-200 ease-out"
                style={{ width: `${progress}%` }}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
