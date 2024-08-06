import { useState, useEffect } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api";
import { listen } from '@tauri-apps/api/event';

import useFileLoader from "./hooks/useFileLoader";
import Overlay from "./components/Overlay"; // Import the Overlay component
import Settings from "./components/Settings"; // Import the Settings component

function App() {
  const { fileFrontmatter, loading } = useFileLoader();
  const [sortOption, setSortOption] = useState("date");
  const [selectedFile, setSelectedFile] = useState(null); // Track the selected file
  const [settingsVisible, setSettingsVisible] = useState(false); // Track the visibility of the settings panel

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

  // Rating dictionary
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

  const sortFiles = (option) => {
    let sortedFiles = [...fileFrontmatter];

    if (option === "date") {
      sortedFiles.sort((a, b) => {
        const dateA = new Date(a.filename.replace(".md", ""));
        const dateB = new Date(b.filename.replace(".md", ""));
        return dateB - dateA;
      });
    } else if (option === "title") {
      sortedFiles.sort((a, b) => {
        return a.entry_title.localeCompare(b.entry_title);
      });
    } else if (option === "week") {
      // Sort by week
      const filesByWeek = {};

      sortedFiles.forEach(file => {
        const date = new Date(file.filename.replace(".md", ""));
        const weekStart = new Date(date);
        weekStart.setDate(date.getDate() - date.getDay());

        const weekStartStr = weekStart.toISOString().split("T")[0];

        if (!filesByWeek[weekStartStr]) {
          filesByWeek[weekStartStr] = new Array(7).fill(null);
        }

        filesByWeek[weekStartStr][date.getDay()] = file;
      });

      const sortedWeeks = Object.keys(filesByWeek).sort((a, b) => new Date(b) - new Date(a));
      const sortedFilesByWeek = {};

      sortedWeeks.forEach(week => {
        sortedFilesByWeek[week] = filesByWeek[week];
      });

      return sortedFilesByWeek;
    }

    return sortedFiles;
  };

  const sortedFileFrontmatter = sortFiles(sortOption);

  const handleFileClick = (file) => {
    setSelectedFile(file);
  };

  const handleOverlayClick = () => {
    setSelectedFile(null);
  };

  const handleSettingsClick = () => {
    setSettingsVisible(!settingsVisible);
  }


  return (
    <div className="container mx-auto p-4">
      <div className="flex justify-end mb-4">
        <label htmlFor="sortOption" className="mr-2">
          Sort By:
        </label>
        <select
          id="sortOption"
          value={sortOption}
          onChange={(e) => setSortOption(e.target.value)}
          className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        >
          <option value="date">Date</option>
          <option value="title">Title</option>
          <option value="week">Week</option>
        </select>
        {/* add a settings button which toggles visablity of the settings panel overlay, it should be gray and have a gear icon */}
        <button className="ml-2 bg-gray-600 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          onClick={handleSettingsClick}
        >
          <span className="sr-only">Settings</span>
          <span role="img" aria-label="settings">⚙️</span> 
        </button>
      </div>
      {sortOption === "week" ? (
        <div className="grid grid-cols-7 gap-4">
          {Object.keys(sortedFileFrontmatter).map(weekStart => (
            <div key={weekStart} className="col-span-7">
              <div className="grid grid-cols-7 gap-4">
                {sortedFileFrontmatter[weekStart].map((file, index) => (
                  file ? (
                    <div
                      key={file.filename}
                      className="flex justify-center items-center relative aspect-[4/3] rounded-lg shadow-lg overflow-hidden transition duration-300 ease-in-out transform hover:scale-105 cursor-pointer"
                      onClick={() => handleFileClick(file)}
                    >
                      <img
                        src={file.entry_thumbnail || "https://pcforms.com/diy-printing-blog/wp-content/uploads/2015/11/fld-step-1a-cut-plain-piece-of-paper-to-size.jpgt1438359447889ampwidth600ampheight338"}
                        alt={file.filename}
                        className="h-full w-full object-cover"
                      />
                      <div className="absolute bottom-0 left-0 bg-black bg-opacity-50 rounded-lg p-2">
                        <p className="text-white text-left">{file.entry_title}</p>
                        <p className="text-white text-sm text-left">
                          {formatDate(file.filename, "short")} {file.hidden && "🔒"}
                        </p>
                      </div>
                      <div className="absolute bottom-0 right-0 bg-black bg-opacity-50 p-2">
                        <p className="text-white text-sm text-right">
                          {file.rating}
                        </p>
                      </div>
                    </div>
                  ) : (
                    <div key={index} className="aspect-[4/3] rounded-lg shadow-lg bg-gray-700">
                      {/* add both vertically and horizontally centered text saying 'no file' */}
                      <div className="flex justify-center items-center h-full text-gray-400 text-1xl">
                        No File
                        </div>
                    </div>
                  )
                ))}
              </div>
            </div>
          ))}
        </div>
      ) : (
        <div className="grid grid-cols-3 gap-4">
          {sortedFileFrontmatter.map((file) => (
            <div
              key={file.filename}
              className="flex justify-center items-center relative aspect-[4/3] rounded-lg shadow-lg overflow-hidden transition duration-300 ease-in-out transform hover:scale-105 cursor-pointer"
              onClick={() => handleFileClick(file)}
            >
              <img
                src={file.entry_thumbnail || "https://pcforms.com/diy-printing-blog/wp-content/uploads/2015/11/fld-step-1a-cut-plain-piece-of-paper-to-size.jpgt1438359447889ampwidth600ampheight338"}
                alt={file.filename}
                className="h-full w-full object-cover"
              />
              <div className="absolute bottom-0 left-0 bg-black bg-opacity-50 rounded-lg p-2">
                <p className="text-white text-left">{file.entry_title}</p>
                <p className="text-white text-sm text-left">
                  {formatDate(file.filename, "short")} {file.hidden && "🔒"}
                </p>
              </div>
              <div className="absolute bottom-0 right-0 bg-black bg-opacity-50 p-2">
                <p className="text-white text-sm text-right">
                  {file.rating}
                </p>
              </div>
            </div>
          ))}
        </div>
      )}
      {selectedFile && (
        <Overlay file={selectedFile} onClick={handleOverlayClick} /> // Render the Overlay component when a file is selected
      )}
      {settingsVisible && (
        <Settings onClick={handleSettingsClick} /> // Render the Settings component when the settings button is clicked
      )}
    </div>
  );
}

export default App;
