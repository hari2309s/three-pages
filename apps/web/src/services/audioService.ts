import api from "@/services/api";
import type { AudioResponse } from "@/types";

export const audioService = {
  getAudio: async (
    summaryId: string,
    language: string,
    voiceType?: string,
  ): Promise<AudioResponse> => {
    const params = new URLSearchParams({ language });
    if (voiceType) {
      params.append("voice_type", voiceType);
    }

    const { data } = await api.get<AudioResponse>(
      `/api/summary/${summaryId}/audio?${params.toString()}`,
    );
    return data;
  },
};
