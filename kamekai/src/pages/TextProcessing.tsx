import { useAuth } from 'react-oidc-context';
import React, { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { ChevronDown, ChevronUp, Home, Copy, Check } from 'lucide-react';
import { ErrorPage } from './ErrorPage';
import { LoadingPage } from './LoadingPage';

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

  const auth = useAuth();

  React.useEffect(() => {
    const fetchTranslations = async () => {
      if (!auth.user?.access_token) {
        setError('No valid authentication token');
        setLoading(false);
        return;
      }

      try {
        const response = await fetch('https://api.seafoodfry.ninja/translate', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${auth.user.access_token}`,
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
  }, [text, auth.user?.access_token]);

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
    return <LoadingPage isLoading={loading} />;
  }

  if (error) {
    return <ErrorPage error={error} onBack={onBack} />;
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
