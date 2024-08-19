import React, { useState } from 'react';
import { CloseSvg, ThreeDotsLoadingSvg, SparkleSvg, CopySvg } from './Icons';

const SoroboyWidget = () => {
    const [expanded, setExpanded] = useState(false);
    const [loading, setLoading] = useState(false);
    const [inputText, setInputText] = useState('');
    const [conversationHistory, setConversationHistory] = useState<
        { userInput: string; response: string }[]
    >([]);
    const [isCopied, setIsCopied] = useState(false);

    const handleButtonClick = () => {
        setExpanded(true);
    };

    const handleInputChange = (e: { target: { value: React.SetStateAction<string>; }; }) => {
        setInputText(e.target.value);
    };

    const handleCloseClick = () => {
        setExpanded(false);
        setLoading(false);
        setInputText('');
    };

    const handleCopyClick = (text: string) => {
        navigator.clipboard.writeText(text);
        setIsCopied(true);
        setTimeout(() => {
            setIsCopied(false);
        }, 2000);
    };

    const handleFormSubmit = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();

        if (!inputText) return;
        setLoading(true);

        const messageHistory = conversationHistory.map(({ userInput, response }) => [
            { role: 'user', content: userInput },
            { role: 'assistant', content: response },
        ]).flat(1);

        console.log(messageHistory);

        const pageContent = document.body.innerText;
        const userInput = inputText;

        try {
            const res = await fetch('https://sandbox.sorobanexamples.xyz/chat', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    model: 'gpt-4o',
                    messages: [
                        {
                            role: 'system',
                            content: `You are a stellar soroban expert named Soroboy! You can help generate smart contracts in context with the current page. you can also help understand soroban in the simple form. Here's the content of the page: ${pageContent}. Parse the page content and only provide relevant responses the prompt. Keep your response short, concise. `,
                        },
                        ...messageHistory,
                        {
                            role: 'user',
                            content: `${userInput}`,
                        }
                    ],
                })
            });

            const data = await res.json();
            const response = data.response.choices[0].message.content;;

            setConversationHistory((prevHistory) => [
                ...prevHistory,
                { userInput, response },
            ]);

        } catch (error) {
            console.error('Error fetching data:', error);
        } finally {
            setLoading(false);
            setInputText('');
        }
    };

    return (
        <div className="fixed bottom-5 right-5 z-50 sm:bottom-3 sm:right-3">
        {!expanded && (
            <button
                onClick={handleButtonClick}
                className="px-4 py-2 bg-[#141414] text-white rounded-md shadow-lg text-sm sm:text-base"
            >
                Generate with AI <span><SparkleSvg className='w-4 h-4 inline' /></span>
            </button>
        )}
        {expanded && (
            <div className="bg-[#141414] py-4 px-3 rounded-lg shadow-lg min-w-[300px] sm:min-w-[400px] max-w-[90vw]">
                <div className="flex justify-between items-center">
                    {!isCopied ? (
                        <span className="text-sm sm:text-base">
                            Soroboy <SparkleSvg className='w-4 h-4 inline-block' />
                        </span>
                    ) : (
                        <span className='text-green-400 text-sm sm:text-base'>Copied to clipboard!</span>
                    )}
                    <button
                        onClick={handleCloseClick}
                        className="text-gray-600 text-lg focus:outline-none"
                    >
                        <CloseSvg className="w-6 h-6" />
                    </button>
                </div>
    
                <div className="conversation-history max-h-[200px] sm:max-h-[300px] overflow-y-auto">
                    {conversationHistory.map((entry, index) => (
                        <div key={index} className="flex flex-col">
                            <p className="py-1 pl-3 pr-2 bg-[#1e1e1e] w-fit rounded self-end text-sm sm:text-base">
                                {entry.userInput}
                            </p>
                            <div className='flex group cursor-pointer'>
                                <span className='mt-1 rounded-full my-auto p-1'>
                                    <SparkleSvg className="w-4 h-4" />
                                </span>
                                <p className="pb-2 pt-0 rounded mt-1 w-fit text-sm sm:text-base">
                                    {entry.response}
                                    <span>
                                        <CopySvg
                                            onClick={() => handleCopyClick(entry.response)}
                                            className='w-4 h-4 inline-block opacity-0 group-hover:opacity-100 transition-opacity'
                                        />
                                    </span>
                                </p>
                            </div>
                            <div className='h-2'></div>
                        </div>
                    ))}
                </div>
    
                <form onSubmit={handleFormSubmit} className="mt-3">
                    <div className="relative">
                        <input
                            type="text"
                            value={inputText}
                            onChange={handleInputChange}
                            className="w-full p-3 bg-[#242424] rounded outline-none text-sm sm:text-base"
                            placeholder="Type something..."
                        />
                        <button
                            type="submit"
                            className="absolute right-2 bottom-1/2 translate-y-1/2 px-2 py-1 bg-[#303030] text-white rounded-md"
                            disabled={loading}
                        >
                            {loading ? (
                                <div className="">
                                    <ThreeDotsLoadingSvg className='w-6 h-6' />
                                </div>
                            ) : "Generate"}
                        </button>
                    </div>
                </form>
            </div>
        )}
    </div>
    
    );
};

export default SoroboyWidget;
