import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { TextProcessing } from './pages/TextProcessing';
import './App.css';

function App() {
  const [inputText, setInputText] = useState('');
  const [showProcessing, setShowProcessing] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (inputText.trim()) {
      setShowProcessing(true);
    }
  };

  const handleBack = () => {
    setShowProcessing(false);
    setInputText('');
  };

  if (showProcessing) {
    return <TextProcessing text={inputText} onBack={handleBack} />;
  }

  return (
    <div className="min-h-screen bg-[#1E1E1E] text-white flex flex-col items-center justify-start">
      <main className="w-full max-w-3xl px-4 py-24 text-center">
        <h1 className="text-5xl font-bold mb-12">Welcome to Kamekai</h1>

        <p className="mb-8">Enter text to get translations in Japanese and Chinese.</p>

        <form className="flex flex-col justify-center gap-4" onSubmit={handleSubmit}>
          <textarea
            value={inputText}
            onChange={e => setInputText(e.target.value)}
            placeholder="What do you want to learn today..."
            className="w-full min-h-[100px] bg-[#2F2F2F] border-gray-600 text-white placeholder-gray-500 rounded-xl focus-visible:ring-blue-500 p-4 resize-none resize-y"
            onKeyDown={e => {
              if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                handleSubmit(e);
              }
            }}
          />
          <Button
            type="submit"
            className="bg-blue-600 hover:bg-blue-700 text-white rounded-xl w-full"
          >
            Translate
          </Button>
        </form>
      </main>
    </div>
  );
}

export default App;
