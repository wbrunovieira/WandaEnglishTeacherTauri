import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

type Props = {
    setMessages: any;
};

function Title({ setMessages }: Props) {
    const [isResetting, setIsResetting] = useState(false);

   
    const resetConversation = async () => {
        setIsResetting(true);

        try {
           
            const response = await invoke("reset_conversation");
            if (response) {
                setMessages([]);
            }
        } catch (err) {
            console.error("Failed to reset conversation:", err);
        }

        setIsResetting(false);
    };

    return (
        <div className="flex justify-between items-center w-full p-4 bg-customPurple1 text-white font-bold shadow">
            <div className="italic">Wanda</div>
            <button
                onClick={resetConversation}
                className={
                    "transition-all duration-300 text-customYellow hover:text-white " +
                    (isResetting && "animate-pulse")
                }
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    strokeWidth={1.5}
                    stroke="currentColor"
                    className="w-6 h-6"
                >
                    <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"
                    />
                </svg>
            </button>
        </div>
    );
}

export default Title;
