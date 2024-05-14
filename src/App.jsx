
import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import { invoke } from "@tauri-apps/api";
import useFileLoader from "./hooks/useFileLoader";  // Import from the correct path

function App() {
  const { fileFrontmatter, loading } = useFileLoader();

  invoke("greet", { name: "World" })
    .then((response) => console.log(response));

  return (
    <>
      {/* <h1 className="text-3xl font-bold underline">Hello world!</h1> */}
      {/* render a red square for each file in the fileFrontmatter array */}
      <div className="grid grid-cols-3 gap-4">
        {fileFrontmatter.map((file) => (
          <div
            key={file.filename}
            className="bg-red-500 h-20 w-20 flex justify-center items-center"
          >
            <p className="text-white">{file.filename}</p>
          </div>
        ))}
      </div>
    </>
  );
}

export default App;