import React from "react";

export default function TitleCardSettings({
    title = "Umhlanga Night",
    date = "2021-09-01",
    rating = "+",
    headerImage = "preview2.jpg",
}) {
    return (
        <div className="mt-4">
            <h3 className="text-md font-semibold text-white">Title Card Settings</h3>
            <div className="relative w-[300px] m-auto" style={{ aspectRatio: "16/9" }}>
                <img
                    src={headerImage}
                    alt="Header image"
                    className="w-full h-full object-cover rounded-md"
                />
                {/* Text overlay */}
                <div className="absolute inset-0 flex flex-col justify-center items-start p-4 bg-black bg-opacity-50 rounded-md">
                    <h4 className="text-white text-lg font-bold">{title}</h4>
                    <p className="text-white text-sm">{date}</p>
                    <p className="text-white text-sm">{rating}</p>
                </div>
            </div>
        </div>
    );
}
