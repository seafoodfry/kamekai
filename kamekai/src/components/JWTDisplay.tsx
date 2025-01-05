import React, { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Copy, Check } from 'lucide-react';

interface JWTDisplayProps {
  token?: string;
  label: string;
}

interface DecodedJWT {
  header: unknown;
  payload: unknown;
}

export const JWTDisplay: React.FC<JWTDisplayProps> = ({ token, label }) => {
  const [showDecoded, setShowDecoded] = useState(false);
  const [copiedText, setCopiedText] = useState<string | null>(null);

  const decodeJWT = (token: string): DecodedJWT => {
    try {
      const [header, payload] = token
        .split('.')
        .slice(0, 2)
        .map(part => JSON.parse(atob(part.replace(/-/g, '+').replace(/_/g, '/'))));
      return { header, payload };
    } catch {
      return { header: 'Invalid token', payload: 'Invalid token' };
    }
  };

  const decoded = token ? decodeJWT(token) : null;

  const handleCopy = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedText(text);
      setTimeout(() => setCopiedText(null), 2000); // Reset after 2 seconds.
    } catch (err) {
      console.error('Failed to copy text: ', err);
    }
  };

  const CopyButton = ({ text }: { text: string }) => (
    <Button
      variant="ghost"
      size="icon"
      className="text-gray-400 hover:text-white h-6 w-6"
      onClick={() => handleCopy(text)}
    >
      {copiedText === text ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
    </Button>
  );

  return (
    <div className="space-y-2">
      <div className="text-xs text-gray-500">
        <div className="flex items-center justify-between">
          <div>{label}:</div>
          <button
            onClick={() => setShowDecoded(!showDecoded)}
            className="text-blue-500 hover:text-blue-400 text-xs"
          >
            {showDecoded ? 'Show Raw' : 'Show Decoded'}
          </button>
        </div>
        {!showDecoded ? (
          <div className="bg-gray-800 p-2 rounded mt-1 break-all font-mono flex items-center justify-between">
            <span className="text-gray-300 flex-grow">{token || 'No token available'}</span>
            <CopyButton text={token || 'No token available'} />
          </div>
        ) : (
          <div className="space-y-2">
            <div>
              <div className="text-blue-400 mt-2">Header:</div>
              <div className="bg-gray-800 p-2 rounded mt-1 break-all font-mono flex items-center justify-between">
                <span className="text-gray-300 flex-grow">
                  {JSON.stringify(decoded?.header, null, 2)}
                </span>
                <CopyButton text={JSON.stringify(decoded?.header, null, 2)} />
              </div>
            </div>
            <div>
              <div className="text-blue-400">Payload:</div>
              <div className="bg-gray-800 p-2 rounded mt-1 break-all font-mono flex items-center justify-between">
                <span className="text-gray-300 flex-grow">
                  {JSON.stringify(decoded?.payload, null, 2)}
                </span>
                <CopyButton text={JSON.stringify(decoded?.payload, null, 2)} />
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
