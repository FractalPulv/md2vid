import React from "react";

const SentenceWithImage = ({
  layout,
  imageSrc,
  text,
  backgroundColor,
  textColor,
}) => {
  const sampleText = "Today I went to the beach";

  const renderLayout = () => {
    switch (layout) {
      case "textBelow":
        return (
          <div className="flex flex-col items-center justify-center h-full">
            <img
              src={imageSrc}
              alt="Image with text below"
              className="w-full h-2/3 object-cover"
            />
            <p className={`mt-2 text-center text-${textColor} text-sm`}>
              {sampleText}
            </p>
          </div>
        );
      case "fullScreenWithCaptions":
        return (
          <div className="relative h-full">
            <img
              src={imageSrc}
              alt="Full screen image"
              className="w-full h-full object-cover"
            />
            <p className="absolute bottom-2 left-1/2 transform -translate-x-1/2 text-white bg-black bg-opacity-50 rounded text-center inline-block text-sm">
              {sampleText}
            </p>
          </div>
        );
      case "blurredBackground":
        return (
          <div className="relative w-full h-full">
            <img
              src={imageSrc}
              alt="Blurred background"
              className="absolute w-full h-full object-cover blur-sm"
            />
            <div className="relative z-10 flex flex-col items-center justify-center h-full">
              <img
                src={imageSrc}
                alt="Small image"
                className="w-1/2 h-1/2 object-cover"
              />
              <p className={`mt-2 text-center text-${textColor} text-sm`}>
                {sampleText}
              </p>
            </div>
          </div>
        );
      default:
        return null;
    }
  };

  return (
    <div className="mt-4 w-full max-w-[300px]">
      {/* Outer div with fixed aspect ratio and consistent size */}
      <div
        className={`relative aspect-w-16 aspect-h-9 bg-${backgroundColor} shadow-md border border-gray-600 rounded-md overflow-hidden`}
        style={{ width: "100%", height: "0", paddingBottom: "56.25%" }} // 16:9 aspect ratio
      >
        <div className="absolute top-0 left-0 w-full h-full">
          {renderLayout()}
        </div>
      </div>
      {/* Div containing the text */}
      <div className="mt-2 text-sm text-white text-center">{text}</div>
    </div>
  );
};

export default SentenceWithImage;
