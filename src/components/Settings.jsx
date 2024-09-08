import { useState, useEffect } from "react";
import TextFormattingComponent from "./TextFormattingComponent";
import SentenceWithImage from "./SentenceWithImage";
import TitleCardSettings from "./TitleCardSettings";

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
      <div className="bg-gray-800  rounded-lg w-full h-full m-40 p-10 overflow-y-auto">
        <h2 className="text-lg font-semibold text-white">Settings</h2>
        <div className="w-96">
          <div className="mt-4">
            <h3 className="text-md font-semibold text-white">
              Markdown File Directory
            </h3>
            <input
              type="text"
              className="border border-gray-500 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white w-full"
            />
          </div>
          <div className="mt-4">
            <h3 className="text-md font-semibold text-white">
              YT DLP Exceptionable Path
            </h3>
            <input
              type="text"
              className="border border-gray-500 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white w-full"
            />
          </div>
          <div className="mt-4">
            <h3 className="text-md font-semibold text-white">
              Local Image Path
            </h3>
            <input
              type="text"
              className="border border-gray-500 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white w-full"
            />
          </div>
          <div className="mt-4">
            <h3 className="text-md font-semibold text-white">
              Obsidian Vault Name
            </h3>
            <input
              type="text"
              className="border border-gray-500 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white w-full"
            />
          </div>
        </div>
        <div className="mt-4 bg-gray-900 p-4 rounded-md">
          <h2>Text Formatting</h2>
          <TextFormattingComponent />
        </div>

        {/* Sentence with Image */}
        <div className="mt-4">
          <h3 className="text-md font-semibold text-white">Text with image </h3>

          {/* Layout: Image with text below */}

          <div className="flex space-x-4 bg-gray-900 p-4 rounded-md flex-wrap justify-center">
            <SentenceWithImage
              layout="textBelow"
              imageSrc="preview.jpg"
              text="Small image"
              backgroundColor="black"
              textColor="text-yellow-500"
            />

            {/* Layout: Full screen image with captions */}
            <SentenceWithImage
              layout="fullScreenWithCaptions"
              imageSrc="preview.jpg"
              text="Full screen image"
            />

            {/* Layout: Image with blurred background */}
            <SentenceWithImage
              layout="blurredBackground"
              imageSrc="preview.jpg"
              text="Blurred background effect"
            />
          </div>
        </div>

        <div className="mt-4">
          <h3 className="text-md font-semibold text-white">Image Resolution</h3>
          <div className="flex items-center space-x-2 w-[300px] m-auto">
            <button className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white flex-1">
              Low
            </button>
            <button className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white flex-1">
              Medium
            </button>
            <button className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white flex-1">
              High
            </button>
          </div>
        </div>

        {/* Title card settings */}
        <TitleCardSettings />

        <div className="mt-4 justify-between">
          <div className="text-white">
            <input type="checkbox" className="mr-2" /> Fade in and out
          </div>
          <div className="text-white">
            <input type="checkbox" className="mr-2" /> Add images
          </div>
          <div className="text-white">
            <input type="checkbox" className="mr-2" /> Let images linger for sentences without images (When checked the last image will appear for subsequent sentences until a new image is found)
          </div>
          <div className="text-white">
            <input type="checkbox" className="mr-2" /> Show title card at the beginning
          </div>
          <div className="text-white">
            <input type="checkbox" className="mr-2" /> Add audio track from video url
          </div>
          <div className="text-white">
            <input type="checkbox" className="mr-2" /> Find embedded video within markdown (Legacy)
          </div>
          <div className="text-white">
            <input type="text" placeholder="totd" className="mr-2" /> Frontmatter youtube URL
          </div>


        </div>
      </div>
    </div>
  );
}
