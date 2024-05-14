import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import { invoke } from "@tauri-apps/api";
import useFileLoader from "./hooks/useFileLoader"; // Import from the correct path

function App() {
  const { fileFrontmatter, loading } = useFileLoader();

  useEffect(() => {
    invoke("greet", { name: "World" }).then((response) =>
      console.log(response)
    );
  }, []);

  const formatDate = (filename) => {
    // Extract the date part of the filename and convert it to a Date object
    const dateString = filename.replace(".md", "");
    const date = new Date(dateString);

    // Format the date to a readable string
    const options = {
      weekday: "short",
      day: "numeric",
      month: "short",
      year: "numeric",
    };
    return date.toLocaleDateString(undefined, options);
  };

  // make a dictionary for the ratings
  // ~ = Average
  // + = Good
  // ++ = Great
  // +++ = Wonderful
  // - = Bad
  // -- = Very Bad
  // --- = Terrible
  // ~+ = Above Average
  // ~- = Below Average
  const ratingDictionary = {
    "~": "Average",
    "+": "Good",
    "++": "Great",
    "+++": "Wonderful",
    "-": "Bad",
    "--": "Very Bad",
    "---": "Terrible",
    "~+": "Above Average",
    "~-": "Below Average",
  };

  return (
    <div className="container mx-auto p-4">
      <div className="grid grid-cols-3 gap-4">
        {fileFrontmatter.map((file) => (
          <div
            key={file.filename}
            className="bg-red-500 flex justify-center items-center relative aspect-[4/3] rounded-lg shadow-lg overflow-hidden transition duration-300 ease-in-out transform hover:scale-105 cursor-pointer"
          >
            <img
              src={
                file.entry_thumbnail
                  ? file.entry_thumbnail
                  : "https://pcforms.com/diy-printing-blog/wp-content/uploads/2015/11/fld-step-1a-cut-plain-piece-of-paper-to-size.jpgt1438359447889ampwidth600ampheight338"
              }
              alt={file.filename}
              className="h-full w-full object-cover"
            />
            <div className="absolute bottom-0 left-0 bg-black bg-opacity-50 rounded-lg p-2">
              <p className="text-white text-left">{file.entry_title}</p>
              <p className="text-white text-sm text-left">
                {formatDate(file.filename)}
              </p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
