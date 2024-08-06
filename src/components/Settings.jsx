import { useState, useEffect } from "react";
import TextFormattingComponent from "./TextFormattingComponent";

export default function Settings({ onClick }) {
  const [visible, setVisible] = useState(false);
  const [boldText, setBoldText] = useState("**Bold text**");
  const [italicText, setItalicText] = useState("*Italic text*");
  const [boldColor, setBoldColor] = useState("#ff0000");
  const [italicColor, setItalicColor] = useState("#00ff00");

  useEffect(() => {
    setVisible(true);
  }, []);

  const handleOverlayClick = (event) => {
    if (event.target === event.currentTarget) {
      setVisible(false);
      setTimeout(onClick, 300); // Delay unmount to allow for fade-out
    }
  };

  const handleBoldChange = (event) => setBoldText(event.target.value);
  const handleItalicChange = (event) => setItalicText(event.target.value);

  return (
    <div
      className={`fixed inset-0 bg-black bg-opacity-70 flex justify-center items-center transition-opacity duration-300 ${
        visible ? "opacity-100" : "opacity-0"
      }`}
      onClick={handleOverlayClick}
    >
      <div className="bg-gray-800 p-4 rounded-lg w-full max-w-2xl h-full max-h-full overflow-y-auto">
        <h2 className="text-lg font-semibold text-white">Settings</h2>
        <div className="mt-4">
          <h3 className="text-md font-semibold text-white">
            Markdown File Directory
          </h3>
          <input
            type="text"
            className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white w-full"
          />
        </div>
        <div className="mt-4">
          <h3 className="text-md font-semibold text-white">
            YT DLP Exceptionable Path
          </h3>
          <input
            type="text"
            className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white w-full"
          />
        </div>
        <div className="mt-4">
          <h3 className="text-md font-semibold text-white">
            Local Image Path
          </h3>
          <input
            type="text"
            className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white w-full"
          />
        </div>
        <div className="mt-4">
          <h3 className="text-md font-semibold text-white">
            Obsidian Vault Name
          </h3>
          <input
            type="text"
            className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white w-full"
          />
        </div>
        <div className="mt-4 bg-gray-900 p-4 rounded-md">
          <h2>Text Formatting</h2>
          <TextFormattingComponent />
        </div>
        <div className="mt-4">
          <h3 className="text-md font-semibold text-white">Italic</h3>
          <div className="flex items-center space-x-2">
            <input
              type="text"
              value={italicText}
              onChange={handleItalicChange}
              className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white flex-1"
            />
            <input
              type="color"
              value={italicColor}
              onChange={(e) => setItalicColor(e.target.value)}
              className="border border-gray-300 rounded-md h-10 w-10"
            />
          </div>
          <div
            className="mt-2 p-2 rounded-md"
            style={{ backgroundColor: italicColor }}
          >
            <em>{italicText}</em>
          </div>
        </div>
        <div className="mt-4">
          <h3 className="text-md font-semibold text-white">Sentence</h3>
          <input
            type="text"
            className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white w-full"
            placeholder="Example sentence"
          />
        </div>
        <div className="mt-4 flex justify-between">
          <div className="text-white">
            <input type="checkbox" className="mr-2" /> Fade in and out
          </div>
          <div className="text-white">
            <input type="checkbox" className="mr-2" /> Fade in and out
          </div>
        </div>
      </div>
    </div>
  );
}
