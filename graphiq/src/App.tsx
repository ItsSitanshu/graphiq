import { getCurrentWindow } from '@tauri-apps/api/window';

// when using `"withGlobalTauri": true`, you may use
// const { getCurrentWindow } = window.__TAURI__.window;

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import "./global.css";

function App() {
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.altKey) {
        e.preventDefault();
        const elements = document.querySelectorAll('._visuals1');
        elements.forEach(el => el.classList.add('underline', 'decoration-white'));
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      if (e.key === 'Alt') {
        const elements = document.querySelectorAll('._visuals1');
        elements.forEach(el => el.classList.remove('underline', 'decoration-white'));
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);
    
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
    };
  }, []);

  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  // async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <main className="container">
    </main>
  );
}

export default App;
