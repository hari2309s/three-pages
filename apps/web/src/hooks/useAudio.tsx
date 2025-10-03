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
  const howlRef = useRef<Howl | null>(null);

  const mutation = useMutation<AudioResponse, Error, AudioMutationArgs>({
    mutationFn: ({ summaryId, language, voiceType }) =>
      audioService.getAudio(summaryId, language, voiceType),
    onSuccess: (data) => {
      if (howlRef.current) {
        howlRef.current.unload();
      }

      howlRef.current = new Howl({
        src: [data.audio_url],
        format: ["wav"],
        onplay: () => setIsPlaying(true),
        onpause: () => setIsPlaying(false),
        onstop: () => {
          setIsPlaying(false);
          setCurrentTime(0);
        },
        onend: () => {
          setIsPlaying(false);
          setCurrentTime(0);
        },
        onload: () => {
          setDuration(howlRef.current?.duration() || 0);
        },
      });
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
    play,
    pause,
    stop,
    seek,
  };
};
