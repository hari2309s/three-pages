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
    retry: 2,
    retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 5000),
    onError: (error) => {
      console.error("Audio generation failed:", error);

      // Set more user-friendly error messages
      if (error.message.includes("timeout")) {
        setAudioError(
          "Audio generation timed out. Please try again with shorter text.",
        );
      } else if (
        error.message.includes("429") ||
        error.message.includes("rate limit")
      ) {
        setAudioError(
          "Audio service is busy. Please wait a moment and try again.",
        );
      } else if (
        error.message.includes("401") ||
        error.message.includes("403")
      ) {
        setAudioError(
          "Authentication error. Please refresh the page and try again.",
        );
      } else if (
        error.message.includes("500") ||
        error.message.includes("502") ||
        error.message.includes("503")
      ) {
        setAudioError(
          "Audio service temporarily unavailable. Please try again later.",
        );
      } else if (
        error.message.includes("network") ||
        error.message.includes("fetch")
      ) {
        setAudioError(
          "Network error. Please check your connection and try again.",
        );
      } else {
        setAudioError(
          "Audio generation failed. Please try again or contact support if the problem persists.",
        );
      }
    },
    onSuccess: (data) => {
      console.log("Audio data received:", {
        id: data.id,
        language: data.language,
        audioUrlLength: data.audio_url?.length || 0,
        hasDataPrefix: data.audio_url?.startsWith("data:") || false,
      });

      // Validate audio URL
      if (!data.audio_url || !data.audio_url.startsWith("data:audio/")) {
        const error =
          "Audio was generated but in an invalid format. Please try again.";
        console.error("Invalid audio format:", {
          audioUrl: data.audio_url?.substring(0, 50),
          hasAudioUrl: !!data.audio_url,
          startsWithData: data.audio_url?.startsWith("data:") || false,
        });
        setAudioError(error);
        return;
      }

      // Additional validation for audio data length
      if (data.audio_url.length < 200) {
        const error = "Audio file appears to be too small. Please try again.";
        console.error("Audio data too small:", {
          length: data.audio_url.length,
        });
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
            const errorMsg =
              "Failed to load audio file. The audio may be corrupted. Please try generating it again.";
            console.error("Howl load error:", {
              id,
              error,
              audioUrlLength: data.audio_url?.length,
            });
            setAudioError(errorMsg);
          },
          onplayerror: (id, error) => {
            const errorMsg =
              "Failed to play audio. Please try refreshing the page and generating the audio again.";
            console.error("Howl play error:", { id, error });
            setAudioError(errorMsg);
            setIsPlaying(false);
          },
        });
      } catch (error) {
        const errorMsg =
          "Failed to initialize audio player. Please try refreshing the page.";
        console.error("Howl creation error:", error);
        setAudioError(errorMsg);
      }
    },
    onMutate: () => {
      // Clear any previous audio errors when starting new generation
      setAudioError(null);
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
    if (!howlRef.current) {
      setAudioError("No audio loaded. Please generate audio first.");
      return;
    }

    try {
      howlRef.current.play();
      setAudioError(null); // Clear any previous playback errors
    } catch (error) {
      console.error("Error playing audio:", error);
      setAudioError("Failed to start playback. Please try again.");
    }
  };

  const pause = () => {
    if (!howlRef.current) return;

    try {
      howlRef.current.pause();
    } catch (error) {
      console.error("Error pausing audio:", error);
    }
  };

  const stop = () => {
    if (!howlRef.current) return;

    try {
      howlRef.current.stop();
      setCurrentTime(0);
    } catch (error) {
      console.error("Error stopping audio:", error);
    }
  };

  const seek = (time: number) => {
    if (!howlRef.current) return;

    try {
      howlRef.current.seek(time);
      setCurrentTime(time);
    } catch (error) {
      console.error("Error seeking audio:", error);
    }
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
