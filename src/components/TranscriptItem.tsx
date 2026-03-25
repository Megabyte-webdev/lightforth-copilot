export default function TranscriptItem({
  speaker,
  text,
  isAi,
}: {
  speaker: string;
  text: string;
  isAi?: boolean;
}) {
  return (
    <div
      className={`group transition-all ${isAi ? "border-l-2 border-cyan-500 pl-3" : "pl-1"}`}
    >
      <div className="flex justify-between items-center mb-1">
        <span
          className={`text-[9px] font-bold uppercase tracking-widest ${isAi ? "text-cyan-400" : "text-slate-500"}`}
        >
          {speaker}
        </span>
        <span className="text-[8px] text-slate-700 group-hover:text-slate-400 uppercase">
          12:44:02
        </span>
      </div>
      <p className="text-sm leading-relaxed text-slate-300 font-light selection:bg-cyan-500/30">
        {text}
      </p>
    </div>
  );
}
