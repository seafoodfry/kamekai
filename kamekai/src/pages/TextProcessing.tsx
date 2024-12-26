import React, { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { ChevronDown, ChevronUp, Home, Copy, Check } from 'lucide-react';

interface Example {
  phrase: string;
  pronunciation: string;
  translation: string;
}

interface LanguageDetails {
  translation: string;
  pronunciation: string;
  grammar: string[];
  examples: Example[];
}

interface Translation {
  original: string;
  japanese: LanguageDetails;
  chinese: LanguageDetails;
}

interface TranslationResponse {
  translations: Translation[];
}

interface TextProcessingProps {
  text: string;
  onBack: () => void;
}

export const TextProcessing: React.FC<TextProcessingProps> = ({ text, onBack }) => {
  const [translations, setTranslations] = useState<Translation[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [expandedCards, setExpandedCards] = useState<Record<number, boolean>>({});
  const [copiedText, setCopiedText] = useState<string | null>(null);

  React.useEffect(() => {
    const fetchTranslations = async () => {
      try {
        const response = await fetch('http://localhost:8080/translate', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ text }),
        });

        if (!response.ok) {
          throw new Error(`Failed to translate (${response.status})`);
        }

        const data = (await response.json()) as TranslationResponse;
        setTranslations(data.translations);
        setError(null);
      } catch (err) {
        setError(`Failed to connect to translation service: ${err}`);
      } finally {
        setLoading(false);
      }
    };

    fetchTranslations();
  }, [text]);

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

  const toggleCard = (index: number) => {
    setExpandedCards(prev => ({
      ...prev,
      [index]: !prev[index],
    }));
  };

  const LanguageSection = ({
    title,
    color,
    details,
  }: {
    title: string;
    color: string;
    details: LanguageDetails;
  }) => (
    <div>
      <div className="flex items-center justify-between mb-2">
        <p className={`text-sm font-semibold ${color}`}>{title}</p>
        <CopyButton text={details.translation} />
      </div>
      <p className="text-md text-gray-200">{details.translation}</p>
      <p className="text-sm text-gray-400 mt-1">{details.pronunciation}</p>
    </div>
  );

  const ExpandedContent = ({ details }: { details: LanguageDetails }) => (
    <div className="mt-4 space-y-4 border-t border-gray-700 pt-4">
      <div>
        <h4 className="text-sm font-semibold text-gray-300 mb-2">Grammar Notes</h4>
        <ul className="list-disc list-inside space-y-1">
          {details.grammar.map((note, i) => (
            <li key={i} className="text-sm text-gray-400">
              {note}
            </li>
          ))}
        </ul>
      </div>

      <div>
        <h4 className="text-sm font-semibold text-gray-300 mb-2">Examples</h4>
        <div className="space-y-3">
          {details.examples.map((example, i) => (
            <div key={i} className="bg-[#252525] p-3 rounded">
              <p className="text-sm text-gray-200">{example.phrase}</p>
              <p className="text-xs text-gray-400 mt-1">{example.pronunciation}</p>
              <p className="text-sm text-gray-300 mt-1">{example.translation}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );

  if (loading) {
    return (
      <div className="min-h-screen bg-[#1E1E1E] text-white flex items-center justify-center">
        <p className="text-xl">Loading translations...</p>
      </div>
    );
  }

  if (error) {
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

                {/* Circuit lines */}
                <line x1="8" y1="20" x2="12" y2="20" />
                <line x1="28" y1="20" x2="32" y2="20" />

                {/* Error symbols */}
                <text x="14" y="12" fontSize="3" fill="currentColor">
                  ?
                </text>
                <text x="24" y="12" fontSize="3" fill="currentColor">
                  !
                </text>

                {/* Static effect lines */}
                <line x1="10" y1="30" x2="14" y2="30" strokeDasharray="1,1" />
                <line x1="26" y1="30" x2="30" y2="30" strokeDasharray="1,1" />
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
  }

  return (
    <div className="min-h-screen bg-[#1E1E1E] text-white">
      <div className="fixed top-4 left-4">
        <Button
          onClick={onBack}
          variant="ghost"
          size="icon"
          className="text-gray-400 hover:text-white hover:bg-[#2A2A2A]"
        >
          <Home className="h-5 w-5" />
        </Button>
      </div>

      <div className="w-full max-w-4xl mx-auto p-6">
        <h2 className="text-4xl font-bold mb-8 text-center bg-gradient-to-r from-red-500 via-yellow-500 to-red-500 bg-clip-text text-transparent">
          Your Text in Multiple Languages
        </h2>

        <div className="space-y-6">
          {translations.map((translation, index) => (
            <Card
              key={index}
              className="bg-[#2A2A2A] border-gray-700 shadow-lg hover:shadow-xl transition-shadow"
            >
              <CardContent className="p-6">
                <div className="space-y-4">
                  <div className="flex items-start justify-between space-x-4">
                    <p className="text-lg text-amber-100 font-medium flex-grow">
                      {translation.original}
                    </p>
                    <CopyButton text={translation.original} />
                  </div>

                  <div className="grid grid-cols-2 gap-6 mt-4">
                    <LanguageSection
                      title="Japanese"
                      color="text-red-400"
                      details={translation.japanese}
                    />
                    <LanguageSection
                      title="Chinese"
                      color="text-yellow-500"
                      details={translation.chinese}
                    />
                  </div>

                  <Button
                    variant="ghost"
                    className="w-full mt-2 text-gray-400 hover:text-white hover:bg-[#252525]"
                    onClick={() => toggleCard(index)}
                  >
                    {expandedCards[index] ? (
                      <>
                        <ChevronUp className="h-4 w-4 mr-2" />
                        Show Less
                      </>
                    ) : (
                      <>
                        <ChevronDown className="h-4 w-4 mr-2" />
                        Show Grammar & Examples
                      </>
                    )}
                  </Button>

                  {expandedCards[index] && (
                    <div className="grid grid-cols-2 gap-6">
                      <ExpandedContent details={translation.japanese} />
                      <ExpandedContent details={translation.chinese} />
                    </div>
                  )}
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </div>
  );
};
