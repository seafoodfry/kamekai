import { useState } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { TextProcessing } from "./pages/TextProcessing";
import "./App.css";


function App() {
  const [inputText, setInputText] = useState("");
  const [showProcessing, setShowProcessing] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (inputText.trim()) {
      setShowProcessing(true);
    }
  };

  const handleBack = () => {
    setShowProcessing(false);
    setInputText("");
  }

  if (showProcessing) {
    return <TextProcessing text={inputText} onBack={handleBack} />;
  }

  return (
    <div className="min-h-screen bg-[#1E1E1E] text-white flex flex-col items-center justify-start">
      <main className="w-full max-w-3xl px-4 py-24 text-center">
        <h1 className="text-5xl font-bold mb-12">Welcome to Kamekai</h1>

        <p className="mb-8">Enter text to get translations in Japanese and Chinese.</p>

        <form className="flex justify-center gap-4" onSubmit={handleSubmit}>
          <Input
            value={inputText}
            onChange={(e) => setInputText(e.currentTarget.value)}
            placeholder="Enter text..."
            className="max-w-[400px] bg-[#2F2F2F] border-gray-600 text-white placeholder-gray-400 rounded-xl focus-visible:ring-blue-500"
          />
          <Button
            type="submit"
            className="bg-blue-600 hover:bg-blue-700 text-white rounded-xl"
          >
            Process
          </Button>
        </form>

    </main>
  </div>
  );
}

export default App;
