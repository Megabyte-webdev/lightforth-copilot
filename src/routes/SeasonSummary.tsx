import { BarChart3, TrendingUp, Calendar } from "lucide-react";

export default function SeasonSummary() {
  const stats = [
    {
      label: "Total Sessions",
      value: "128",
      icon: Calendar,
      color: "text-blue-400",
    },
    {
      label: "Avg. Accuracy",
      value: "92.4%",
      icon: Target,
      color: "text-emerald-400",
    },
    {
      label: "Peak Performance",
      value: "98%",
      icon: TrendingUp,
      color: "text-purple-400",
    },
  ];

  return (
    <div className="animate-in fade-in slide-in-from-bottom-2 duration-500">
      <header className="mb-8">
        <h1 className="text-3xl font-bold text-white italic tracking-tight">
          SEASON SUMMARY
        </h1>
        <p className="text-slate-400">
          Historical performance and cycle analytics.
        </p>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-10">
        {stats.map((stat, i) => (
          <div
            key={i}
            className="bg-slate-900/50 border border-slate-800 p-6 rounded-2xl flex items-center justify-between"
          >
            <div>
              <p className="text-slate-500 text-xs uppercase tracking-widest mb-1">
                {stat.label}
              </p>
              <h3 className="text-2xl font-mono font-bold text-white">
                {stat.value}
              </h3>
            </div>
            <stat.icon className={`${stat.color} opacity-80`} size={28} />
          </div>
        ))}
      </div>

      <div className="bg-slate-900/30 border border-slate-800 rounded-2xl p-8 h-64 flex flex-col items-center justify-center text-center">
        <BarChart3 className="text-slate-700 mb-4" size={48} />
        <p className="text-slate-500 font-medium">Activity Graph Placeholder</p>
        <p className="text-slate-600 text-sm italic">
          Connect your data source to visualize the season trend.
        </p>
      </div>
    </div>
  );
}

// Quick Icon fallback if Lucide Target isn't imported
function Target({ className, size }: { className?: string; size?: number }) {
  return (
    <svg
      className={className}
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <circle cx="12" cy="12" r="10" />
      <circle cx="12" cy="12" r="6" />
      <circle cx="12" cy="12" r="2" />
    </svg>
  );
}
