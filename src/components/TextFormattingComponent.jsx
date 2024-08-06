import React, { useState, useEffect, useRef } from "react";

// src/components/TextFormattingComponent.jsx
export default function TextFormattingComponent({
  defaultText = "**Formatted text**",
  defaultInputs = [
    { markdownSyntax: "**Formatted Text**" },
    { markdownSyntax: "__Formatted Text__" },
  ],
  formatType = "Bold",
  onSave,
}) {
  const [formattedText, setFormattedText] = useState(defaultText);
  const [textColor, setTextColor] = useState("#ffffff");
  const [highlightColor, setHighlightColor] = useState("transparent");
  const [activeButtons, setActiveButtons] = useState([]);
  const [markdownSyntaxInputs, setMarkdownSyntaxInputs] =
    useState(defaultInputs);
  const inputContainerRef = useRef(null);
  const previewContainerRef = useRef(null);

  const handleTextChange = (event) => setFormattedText(event.target.value);
  const handleTextColorChange = (event) => setTextColor(event.target.value);
  const handleHighlightColorChange = (event) =>
    setHighlightColor(event.target.value);

  const addMarkdownSyntaxInput = () => {
    setMarkdownSyntaxInputs([...markdownSyntaxInputs, { markdownSyntax: "" }]);
  };

  const updateMarkdownSyntaxInput = (index, value) => {
    const updatedInputs = [...markdownSyntaxInputs];
    updatedInputs[index].markdownSyntax = value;
    setMarkdownSyntaxInputs(updatedInputs);
  };

  const removeMarkdownSyntaxInput = () => {
    if (markdownSyntaxInputs.length > 1) {
      setMarkdownSyntaxInputs(markdownSyntaxInputs.slice(0, -1));
    }
  };

  const translateMarkdownToPreview = (markdown) => {
    return markdown
      .replace(/\*\*(.*?)\*\*/g, "<strong>$1</strong>")
      .replace(/__(.*?)__/g, "<u>$1</u>")
      .replace(/\*(.*?)\*/g, "<em>$1</em>")
      .replace(/_(.*?)_/g, "<em>$1</em>")
      .replace(/~~(.*?)~~/g, "<del>$1</del>")
      .replace(/`(.*?)`/g, "<code>$1</code>");
  };

  const toggleButton = (format) => {
    let newText;
    const escapedFormat = format.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    const regex = new RegExp(`(${escapedFormat})(.*?)(${escapedFormat})`, "g");

    if (formattedText.match(regex)) {
      newText = formattedText.replace(regex, "$2");
      setActiveButtons(activeButtons.filter((btn) => btn !== format));
    } else {
      newText = `${format}${formattedText}${format}`;
      setActiveButtons([...activeButtons, format]);
    }
    setFormattedText(newText);
  };

  const getContrastingColor = (hexColor) => {
    const r = parseInt(hexColor.slice(1, 3), 16);
    const g = parseInt(hexColor.slice(3, 5), 16);
    const b = parseInt(hexColor.slice(5, 7), 16);
    const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
    return luminance > 0.5 ? "#000000" : "#ffffff";
  };

  const textContrastingColor = getContrastingColor(textColor);
  const highlightContrastingColor = getContrastingColor(highlightColor);

  useEffect(() => {
    const inputHeight = inputContainerRef.current.clientHeight;
    previewContainerRef.current.style.height = `${inputHeight}px`;
  }, [markdownSyntaxInputs]);

  return (
    <div className="mt-4 bg-gray-900 p-4 rounded-md">
      <h3 className="text-md font-semibold text-white">{formatType}</h3>
      <div className="mt-4 flex items-center space-x-2 bg-gray-800 p-3 rounded-md justify-center mb-4">
        <button
          onClick={() => toggleButton("**")}
          className={`px-3 py-2 rounded-md ${
            activeButtons.includes("**") ? "bg-blue-700" : "bg-gray-700"
          } text-white`}
        >
          B
        </button>
        <button
          onClick={() => toggleButton("*")}
          className={`px-3 py-2 rounded-md ${
            activeButtons.includes("*") ? "bg-blue-700" : "bg-gray-700"
          } text-white italic`}
        >
          I
        </button>
        <button
          onClick={() => toggleButton("__")}
          className={`px-3 py-2 rounded-md ${
            activeButtons.includes("__") ? "bg-blue-700" : "bg-gray-700"
          } text-white underline`}
        >
          U
        </button>
        <button
          onClick={() => toggleButton("~~")}
          className={`px-3 py-2 rounded-md ${
            activeButtons.includes("~~") ? "bg-blue-700" : "bg-gray-700"
          } text-white`}
        >
          <strike>S</strike>
        </button>
        <div className="relative inline-block">
          <span
            className="absolute inset-0 flex items-center justify-center pointer-events-none bg-gray-700 rounded-md"
            style={{ color: textColor }}
          >
            A
          </span>
          <input
            type="color"
            value={textColor}
            onChange={handleTextColorChange}
            className="h-10 w-10 opacity-0"
            title="Text Color"
          />
        </div>
        <div className="relative inline-block">
          <span
            className="absolute inset-0 flex items-center justify-center pointer-events-none bg-gray-700 rounded-md"
            style={{
              color: highlightContrastingColor,
              backgroundColor: highlightColor,
            }}
          >
            H
          </span>
          <input
            type="color"
            value={highlightColor}
            onChange={handleHighlightColorChange}
            className="h-10 w-10 opacity-0"
            title="Highlight Color"
          />
        </div>
        {highlightColor !== "transparent" && (
          <div className="relative inline-block">
            <button
              onClick={() => setHighlightColor("transparent")}
              className="text-white px-1 py-1 rounded-md bg-blue-500"
            >
              ⟲
            </button>
          </div>
        )}
      </div>
      <div className="flex flex-col md:flex-row items-start md:items-start space-x-0 md:space-x-4 space-y-4 md:space-y-0">
        <div
          className="flex flex-col space-y-2 flex-1  min-h-[200px]"
          ref={inputContainerRef}
        >
          <span className="text-gray-400 mb-2">Inputs</span>
          {markdownSyntaxInputs.map((input, index) => (
            <input
              key={index}
              type="text"
              value={input.markdownSyntax}
              className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-700 text-white"
              onChange={(e) => updateMarkdownSyntaxInput(index, e.target.value)}
            />
          ))}
          <div className="flex space-x-2 bottom-0">
            <button
              onClick={addMarkdownSyntaxInput}
              className="bg-blue-500 text-white px-3.5 py-2 rounded-md text-lg font-bold text-center"
            >
              +
            </button>
            <button
              onClick={removeMarkdownSyntaxInput}
              className="bg-red-500 text-white px-3 py-2 rounded-md"
            >
              —
            </button>
          </div>
        </div>
        <div
          className="flex-1 flex flex-col justify-between"
          ref={previewContainerRef}
        >
          <div className="flex flex-col h-full">
            <span className="text-gray-400 mb-1.5 block">Preview</span>
            <div className="flex flex-col h-full flex-grow">
              <div className="mt-2 p-4 rounded-md bg-gray-800 flex-grow flex items-center">
                <span
                  dangerouslySetInnerHTML={{
                    __html: translateMarkdownToPreview(formattedText),
                  }}
                  style={{ color: textColor, backgroundColor: highlightColor }}
                  className="p-3 rounded-md block w-full"
                ></span>
              </div>
            </div>
          </div>
          <div className="mt-4 flex space-x-2 justify-end">
            <button
              onClick={() => setFormattedText(defaultText)}
              className="bg-red-500 text-white px-3 py-2 rounded-md"
            >
              Discard Changes
            </button>
            <button
              onClick={() => onSave(formattedText)}
              className="bg-green-500 text-white px-3 py-2 rounded-md"
            >
              Save
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
