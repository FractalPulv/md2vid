import React, { useEffect, useState, useRef } from "react";
import useVideoLoader from "../hooks/useVideoLoader";
import useFileContentLoader from "../hooks/useFileContentLoader";
import useEventListeners from "../hooks/useEventListeners";
import { formatDate } from "../hooks/useFormatDate";

const Overlay = ({ file, onClick }) => {
  const [visible, setVisible] = useState(false);
  const imageRef = useRef(null);
  const [progress, setProgress] = useState(0);
  const [stage, setStage] = useState("");
  const [videoReady, setVideoReady] = useState(false);

  const createVideo = useVideoLoader(file);
  const getTextContent = useFileContentLoader(file);

  useEventListeners(setProgress, setStage);

  useEffect(() => {
    if (stage === "Done") {
      setVideoReady(true);
    }
  }, [stage]);

  useEffect(() => {
    setVisible(true);
    return () => setVisible(false);
  }, [file]);

  const handleOverlayClick = (event) => {
    if (event.target === event.currentTarget) {
      setVisible(false);
      setTimeout(onClick, 300); // Delay unmount to allow for fade-out
    }
  };

  if (!file) {
    return null;
  }



  const maskBorderStyle = {
    position: "absolute",
    top: "50%",
    left: "50%",
    width: "80%",
    height: "50%",
    border: "4px solid white",
    boxSizing: "border-box",
    transform: "translate(-50%, -50%)",
    pointerEvents: "none",
    zIndex: 1,
    maskImage: "url({{file.entry_thumbnail}})",
  };

  const headerImageStyle = {
    position: "relative",
    overflow: "hidden",
    width: "100%",
    height: "300px",
  };
  const moveImageAnimation = `
        @keyframes moveImage {
            0% {
                transform: translateY(0);
            }
            50% {
                transform: translateY(-30%);
            }
            100% {
                transform: translateY(0);
            }
        }
    `;

  const imageStyle = {
    width: "100%",
    height: "auto",
    clipPath: "inset(0)",
    animation: "moveImage 10s infinite",
    animationTimingFunction: "ease-in-out",
    animationPlayState: "running",
  };

  const videoContainerStyle = {
    position: "absolute",
    top: "50%",
    left: "50%",
    transform: "translate(-50%, -50%)",
    zIndex: 2,
  };

  return (
    <div
      className={`fixed inset-0 bg-black bg-opacity-70 flex justify-center items-center transition-opacity duration-300 ${
        visible ? "opacity-100" : "opacity-0"
      }`}
      onClick={handleOverlayClick}
    >
      <div className="bg-gray-800 rounded-lg shadow-lg relative p-1 m-20">
        <button
          className="absolute top-2 right-2 text-white bg-red-500 p-2 rounded-full"
          onClick={() => {
            setVisible(false);
            setTimeout(onClick, 300); // Delay unmount to allow for fade-out
          }}
        >
          Close
        </button>
        <div className="flex justify-center items-start mb-4">
          <div style={headerImageStyle}>
            <div style={maskBorderStyle}></div>
            <img
              ref={imageRef}
              src={
                file.entry_thumbnail ||
                "https://pcforms.com/diy-printing-blog/wp-content/uploads/2015/11/fld-step-1a-cut-plain-piece-of-paper-to-size.jpgt1438359447889ampwidth600ampheight338"
              }
              alt={file.filename}
              style={imageStyle}
            />
            <div className="absolute bottom-0 left-0 p-4  text-white bg-gradient-to-t from-gray-800 to-transparent w-full">
              <h2 className="text-2xl font-bold mb-2">{file.entry_title}</h2>
              <p className="mb-2">
                {formatDate(file.filename, "long")}{file.hidden && "🔒"}
              </p>
              <p className="mb-2">Rating: {file.rating}</p>
              <div
                dangerouslySetInnerHTML={{ __html: file.html }}
                className="content"
              ></div>
            </div>
          </div>
        </div>
        <button
          onClick={createVideo}
          className="bg-blue-500 text-white px-3 py-2 rounded-md m-10"
        >
          Generate Video
        </button>
        {/* <button
          onClick={getTextContent}
          className="bg-blue-500 text-white px-3 py-2 rounded-md m-10"
        >
          Get Text Content
        </button> */}
        <br />
        <span className="mb-2">
          {stage}</span>
        <div className="progress-bar">
          <div
            className="progress-bar-fill"
            style={{ width: `${progress}%` }}
          ></div>
        </div>
        {progress === 100 && videoReady && (
          <div
            style={{
              position: "absolute",
              top: 0,
              left: 0,
              width: "100%",
              height: "100%",
              zIndex: 2,
              backgroundColor: "rgba(0, 0, 0, 0.5)",
            }}
          >
            <video
              src="./src-tauri/final_output.mp4"
              controls
              allowFullScreen
              style={{
                position: "absolute",
                top: "50%",
                left: "50%",
                transform: "translate(-50%, -50%)",
              }}
            />
          </div>
        )}
      </div>
      <style>{moveImageAnimation}</style>
      <style>
        {`
          .progress-bar {
            width: 100%;
            height: 10px;
            background-color: #f3f3f3;
            border-radius: 5px;
            overflow: hidden;
          }
          
          .progress-bar-fill {
            height: 100%;
            background-color: #4a90e2;
            transition: width 0.3s ease-in-out;
          }
        `}
      </style>
    </div>
  );
};

export default Overlay;
