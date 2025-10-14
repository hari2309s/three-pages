import { SUMMARY_STYLES } from "@/lib/constants";

interface StyleSelectorProps {
  selectedStyle: string;
  onStyleChange: (style: string) => void;
}

export const StyleSelector = ({
  selectedStyle,
  onStyleChange,
}: StyleSelectorProps) => {
  return (
    <div className="flex-1">
      <label htmlFor="style" className="mb-2 block text-sm font-medium">
        Summary Style
      </label>
      <select
        id="style"
        value={selectedStyle}
        onChange={(e) => onStyleChange(e.target.value)}
        className="w-full rounded-lg border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
      >
        {SUMMARY_STYLES.map((style) => (
          <option key={style.value} value={style.value}>
            {style.label} - {style.description}
          </option>
        ))}
      </select>
    </div>
  );
};
