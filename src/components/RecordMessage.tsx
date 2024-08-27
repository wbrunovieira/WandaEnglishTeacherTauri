import { useState } from "react";
import RecordIcon from "./RecordIcon";

type Props = {
    handleStop: () => void;
};

const RecordMessage = ({ handleStop }: Props) => {
    const [isRecording, setIsRecording] = useState(false);

    const startRecording = () => {
        setIsRecording(true);
        // Aqui você pode iniciar a gravação diretamente chamando handleStop após um tempo, se necessário
    };

    const stopRecording = () => {
        setIsRecording(false);
        handleStop(); // Chama o handleStop para processar o áudio
    };

    return (
        <div className="mt-2">
            <button
                onMouseDown={startRecording}
                onMouseUp={stopRecording}
                className="bg-white p-4 rounded-full"
            >
                <RecordIcon
                    classText={
                        isRecording
                            ? "animate-pulse text-red-500"
                            : "text-customYellow"
                    }
                />
            </button>
            <p className="mt-2 text-white font-light">
                {isRecording ? "Recording..." : "Click to Record"}
            </p>
        </div>
    );
};

export default RecordMessage;
