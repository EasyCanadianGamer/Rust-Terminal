"use client";

import { useEffect, useRef, useState } from "react";

export function Terminal() {
  const [lines, setLines] = useState<string[]>([]);
  const [currentInput, setCurrentInput] = useState("");
  const [ws, setWs] = useState<WebSocket | null>(null);
  const terminalRef = useRef<HTMLDivElement>(null);
  const prompt = "user@device <";

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:8080/ws");

    socket.onopen = () => {
      console.log("WebSocket connected");
      setWs(socket);
    };

    socket.onmessage = (event) => {
      setLines((prev) => [...prev, event.data, ""]);
    };
    socket.onmessage = (event) => {
        const data = event.data;
      
        if (data === "\x1B[2J\x1B[1;1H") {
          // Clear the terminal output on 'clear' command
          setLines([]);
        } else {
          setLines((prev) => [...prev, data, ""]);
        }
      };

    socket.onclose = () => {
      console.log("WebSocket disconnected");
      setWs(null);
    };

    socket.onerror = (err) => {
      console.error("WebSocket error:", err);
    };

    return () => {
      socket.close();
    };
  }, []);

  const handleCommand = (command: string) => {
    if (ws && ws.readyState === WebSocket.OPEN) {
      setLines((prev) => [...prev, `${prompt} ${command}`]);
      ws.send(command);
    } else {
      setLines((prev) => [...prev, `${prompt} ${command}`, "Error: Not connected to backend.", ""]);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key === "Enter") {
      e.preventDefault();
      const trimmed = currentInput.trim();
      if (trimmed !== "") {
        handleCommand(trimmed);
        setCurrentInput("");
      }
    } else if (e.key === "Backspace") {
      setCurrentInput((prev) => prev.slice(0, -1));
    } else if (e.key.length === 1) {
      setCurrentInput((prev) => prev + e.key);
    }
  };

  useEffect(() => {
    terminalRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [lines]);

  return (
    <div
      tabIndex={0}
      onKeyDown={handleKeyDown}
      style={{
        backgroundColor: "black",
        color: "white",
        fontFamily: "monospace",
        padding: "1rem",
        height: "100vh",
        width: "100%",
        outline: "none",
        overflowY: "auto",
        whiteSpace: "pre-wrap",
        cursor: "text",
      }}
    >
      {lines.map((line, index) => (
        <div key={index}>{line}</div>
      ))}
      <div>
        {prompt} {currentInput}
        <span className="blinking-cursor">█</span>
      </div>
      <div ref={terminalRef} />
      <style>{`
        .blinking-cursor {
          animation: blink 1s step-start infinite;
        }
        @keyframes blink {
          50% { opacity: 0; }
        }
      `}</style>
    </div>
  );
}
