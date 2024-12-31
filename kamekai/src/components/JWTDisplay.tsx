import React, { useState } from 'react';

interface JWTDisplayProps {
  token?: string;
}

interface DecodedJWT {
  header: unknown;
  payload: unknown;
}

export const JWTDisplay: React.FC<JWTDisplayProps> = ({ token }) => {
  const [showDecoded, setShowDecoded] = useState(false);

  const decodeJWT = (token: string): DecodedJWT => {
    try {
      const [header, payload] = token
        .split('.')
        .slice(0, 2)
        .map(part => JSON.parse(atob(part.replace(/-/g, '+').replace(/_/g, '/'))));
      return { header, payload };
    } catch {
      // Using underscore prefix to indicate intentionally unused parameter
      return { header: 'Invalid token', payload: 'Invalid token' };
    }
  };

  const decoded = token ? decodeJWT(token) : null;

  return (
    <div className="space-y-2">
      <div className="text-xs text-gray-500">
        <div className="flex items-center justify-between">
          <div>ID Token:</div>
          <button
            onClick={() => setShowDecoded(!showDecoded)}
            className="text-blue-500 hover:text-blue-400 text-xs"
          >
            {showDecoded ? 'Show Raw' : 'Show Decoded'}
          </button>
        </div>
        {!showDecoded ? (
          <div className="bg-gray-800 p-2 rounded mt-1 break-all font-mono">
            {token || 'No token available'}
          </div>
        ) : (
          <div className="space-y-2">
            <div>
              <div className="text-blue-400 mt-2">Header:</div>
              <div className="bg-gray-800 p-2 rounded mt-1 break-all font-mono">
                {JSON.stringify(decoded?.header, null, 2)}
              </div>
            </div>
            <div>
              <div className="text-blue-400">Payload:</div>
              <div className="bg-gray-800 p-2 rounded mt-1 break-all font-mono">
                {JSON.stringify(decoded?.payload, null, 2)}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
