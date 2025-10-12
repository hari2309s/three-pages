import { useState, useEffect, useRef } from "react";
import { useMutation } from "@tanstack/react-query";
import { Howl } from "howler";
import { audioService } from "@/services/audioService";
import type { AudioResponse } from "@/types";

interface AudioMutationArgs {
  summaryId: string;
  language: string;
  voiceType?: string;
}

export const useAudio = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [audioError, setAudioError] = useState<string | null>(null);
  const howlRef = useRef<Howl | null>(null);

  const mutation = useMutation<AudioResponse, Error, AudioMutationArgs>({
    mutationFn: ({ summaryId, language, voiceType }) =>
      audioService.getAudio(summaryId, language, voiceType),
    onSuccess: (data) => {
      console.log("Audio data received:", {
        id: data.id,
        language: data.language,
        audioUrlLength: data.audio_url?.length || 0,
        hasDataPrefix: data.audio_url?.startsWith("data:") || false,
      });

      // Validate audio URL
      if (!data.audio_url || !data.audio_url.startsWith("data:audio/")) {
        const error = "Invalid audio data format received from server";
        console.error(error, { audioUrl: data.audio_url?.substring(0, 50) });
        setAudioError(error);
        return;
      }

      if (howlRef.current) {
        howlRef.current.unload();
      }

      setAudioError(null);

      try {
        howlRef.current = new Howl({
          src: [data.audio_url],
          format: ["wav", "mp3", "ogg"],
          html5: true,
          onplay: () => {
            console.log("Audio playback started");
            setIsPlaying(true);
          },
          onpause: () => {
            console.log("Audio playback paused");
            setIsPlaying(false);
          },
          onstop: () => {
            console.log("Audio playback stopped");
            setIsPlaying(false);
            setCurrentTime(0);
          },
          onend: () => {
            console.log("Audio playback ended");
            setIsPlaying(false);
            setCurrentTime(0);
          },
          onload: () => {
            const audioDuration = howlRef.current?.duration() || 0;
            console.log("Audio loaded successfully, duration:", audioDuration);
            setDuration(audioDuration);
          },
          onloaderror: (id, error) => {
            const errorMsg = `Failed to load audio: ${error}`;
            console.error(errorMsg, { id, error });
            setAudioError(errorMsg);
          },
          onplayerror: (id, error) => {
            const errorMsg = `Failed to play audio: ${error}`;
            console.error(errorMsg, { id, error });
            setAudioError(errorMsg);
            setIsPlaying(false);
          },
        });
      } catch (error) {
        const errorMsg = `Error creating Howl instance: ${error}`;
        console.error(errorMsg);
        setAudioError(errorMsg);
      }
    },
  });

  useEffect(() => {
    const interval = setInterval(() => {
      if (howlRef.current && isPlaying) {
        setCurrentTime(howlRef.current.seek());
      }
    }, 100);

    return () => clearInterval(interval);
  }, [isPlaying]);

  const play = () => {
    howlRef.current?.play();
  };

  const pause = () => {
    howlRef.current?.pause();
  };

  const stop = () => {
    howlRef.current?.stop();
  };

  const seek = (time: number) => {
    howlRef.current?.seek(time);
    setCurrentTime(time);
  };

  useEffect(() => {
    return () => {
      if (howlRef.current) {
        howlRef.current.unload();
      }
    };
  }, []);

  return {
    ...mutation,
    isPlaying,
    currentTime,
    duration,
    audioError,
    play,
    pause,
    stop,
    seek,
    reset: () => {
      if (howlRef.current) {
        howlRef.current.unload();
        howlRef.current = null;
      }
      setIsPlaying(false);
      setCurrentTime(0);
      setDuration(0);
      setAudioError(null);
      mutation.reset();
    },
  };
};
