import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import "./App.css";


function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="min-h-screen bg-[#1E1E1E] text-white flex flex-col items-center justify-start">
      <main className="w-full max-w-3xl px-4 py-24 text-center">
        <h1 className="text-5xl font-bold mb-12">Welcome to Tauri + React</h1>

        <div className="flex justify-center items-center space-x-8 mb-12">
          <a href="https://vitejs.dev" target="_blank" className="hover:scale-110 transition-transform">
            <img src="/vite.svg" className="logo vite w-24 h-24" alt="Vite logo" />
          </a>
          <a href="https://tauri.app" target="_blank" className="hover:scale-110 transition-transform">
            <img src="/tauri.svg" className="logo vite w-24 h-24" alt="Tauri logo" />
          </a>
          <a href="https://reactjs.org" target="_blank" className="hover:scale-110 transition-transform">
            <img src={reactLogo} className="logo vite w-24 h-24" alt="React logo" />
          </a>
        </div>
        <p className="mb-4">Click on the Tauri, Vite, and React logos to learn more.</p>

        <form
          className="flex justify-center gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
        <Input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
          className="max-w-[240px] bg-[#2F2F2F] border-gray-600 text-white placeholder-gray-400 rounded-xl focus-visible:ring-blue-500"
        />
        <Button
          type="submit"
          size="default"
          className="bg-blue-600 hover:bg-blue-700 text-white rounded-xl"
        >
          Greet
        </Button>
      </form>
      {greetMsg && <p className="text-xl mt-8">{greetMsg}</p>}
    </main>
  </div>
  );
}

export default App;
