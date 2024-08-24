// VideoModal.tsx
import React, { useState } from 'react';

interface VideoModalProps {
  videoId: string;
}

export const VideoModal: React.FC<VideoModalProps> = ({ videoId }) => {
  const [isOpen, setIsOpen] = useState(false);

  const handleOpen = () => {
    setIsOpen(true);
  };

  const handleClose = () => {
    setIsOpen(false);
  };

  return (
    <div>
      <button
        className="border-black dark:border-white border border-solid border-opacity-10 dark:border-opacity-10 rounded-lg h-full px-5 py-3"
        onClick={handleOpen}
      >
        Play Video
      </button>
      {isOpen && (
        <div
          className="p-4 md:p-0 fixed top-0 right-0 bottom-0 left-0 bg-gray-900 bg-opacity-75 flex items-center justify-center z-30"
          onClick={handleClose}
        >
          <div
            className="bg-[--vocs-color_background] rounded-lg shadow-lg w-full md:w-1/2 h-2/6 lg:h-1/2"
            onClick={(e) => e.stopPropagation()}
          >
            <iframe
              src={`https://www.youtube.com/embed/${videoId}`}
              allowFullScreen
              className="w-full h-full rounded-lg"
            />
          </div>
        </div>
      )}
    </div>
  );
};