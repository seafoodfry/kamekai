import React from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Home, Copy, Check } from 'lucide-react';

interface Sentence {
  original: string;
  chinese: string;
  japanese: string;
}

interface TextProcessingProps {
  text: string;
  onBack: () => void;
}

const mockProcessText = (text: string): Sentence[] => {
  // Simple sentence splitting on periods.
  const sentences = text.split('.').filter(s => s.trim().length > 0);

  return sentences.map(sentence => ({
    original: sentence.trim(),
    chinese: `[CN] ${sentence.trim()}`, // Mock translation
    japanese: `[JP] ${sentence.trim()}`, // Mock translation
  }));
};

export const TextProcessing: React.FC<TextProcessingProps> = ({ text, onBack }) => {
  const sentences = mockProcessText(text);
  const [copiedText, setCopiedText] = React.useState<string | null>(null);

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
          {sentences.map((sentence, index) => (
            <Card
              key={index}
              className="bg-[#2A2A2A] border-gray-700 shadow-lg hover:shadow-xl transition-shadow"
            >
              <CardContent className="p-6">
                <div className="space-y-4">
                  <div className="flex items-start justify-between space-x-4">
                    <p className="text-lg text-amber-100 font-medium flex-grow">
                      {sentence.original}
                    </p>
                    <CopyButton text={sentence.original} />
                  </div>

                  <div className="grid grid-cols-2 gap-6 mt-4">
                    <div>
                      <div className="flex items-center justify-between mb-2">
                        <p className="text-sm text-red-400 font-semibold">Japanese</p>
                        <CopyButton text={sentence.japanese} />
                      </div>
                      <p className="text-md text-gray-200">{sentence.japanese}</p>
                    </div>

                    <div>
                      <div className="flex items-center justify-between mb-2">
                        <p className="text-sm text-yellow-500 font-semibold">Chinese</p>
                        <CopyButton text={sentence.chinese} />
                      </div>
                      <p className="text-md text-gray-200">{sentence.chinese}</p>
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </div>
  );
};
