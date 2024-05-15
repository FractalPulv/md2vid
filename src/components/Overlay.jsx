import React, { useEffect, useState, useRef } from "react";

const Overlay = ({ file, onClick }) => {
    const [visible, setVisible] = useState(false);
    const imageRef = useRef(null);

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

    return (
        <div
            className={`fixed inset-0 bg-black bg-opacity-70 flex justify-center items-center transition-opacity duration-300 ${
                visible ? "opacity-100" : "opacity-0"
            }`}
            onClick={handleOverlayClick}
        >
            <div className="bg-gray-800 rounded-lg shadow-lg relative p-0 m-20">
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
                                src={file.entry_thumbnail || "https://pcforms.com/diy-printing-blog/wp-content/uploads/2015/11/fld-step-1a-cut-plain-piece-of-paper-to-size.jpgt1438359447889ampwidth600ampheight338"}
                                alt={file.filename}
                                style={imageStyle}
                            />
                        
                    </div>
                </div>
                <h2 className="text-2xl font-bold mb-2 text-white">
                    {file.entry_title}
                </h2>
                <p className="mb-2 text-white">{file.filename}</p>
                <p className="mb-2 text-white">Rating: {file.rating}</p>
                <div
                    dangerouslySetInnerHTML={{ __html: file.html }}
                    className="content text-white"
                ></div>
            </div>
            <style>
                {moveImageAnimation}
            </style>
        </div>
    );
};

export default Overlay;
