import { Filter, MoreHorizontal, Download } from "lucide-react";

export default function Sheets() {
  // Mock data for the table
  const rows = [
    {
      id: "SH-001",
      name: "Initial Assessment",
      date: "2026-03-24",
      status: "Completed",
    },
    {
      id: "SH-002",
      name: "Mid-Season Calibration",
      date: "2026-03-25",
      status: "Active",
    },
    {
      id: "SH-003",
      name: "Tactical Review",
      date: "2026-03-27",
      status: "Pending",
    },
  ];

  return (
    <div className="animate-in fade-in slide-in-from-bottom-2 duration-500">
      <header className="flex justify-between items-end mb-8">
        <div>
          <h1 className="text-3xl font-bold text-white">CURRENT SHEETS</h1>
          <p className="text-slate-400">
            Manage and export active data records.
          </p>
        </div>

        <div className="flex gap-2">
          <button className="p-2 bg-slate-800 text-slate-300 rounded-lg hover:bg-slate-700 transition-colors">
            <Filter size={18} />
          </button>
          <button className="flex items-center gap-2 bg-slate-100 text-slate-900 px-4 py-2 rounded-lg font-semibold hover:bg-white transition-colors">
            <Download size={18} /> Export CSV
          </button>
        </div>
      </header>

      <div className="bg-slate-900/50 border border-slate-800 rounded-xl overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full text-left border-collapse">
            <thead>
              <tr className="border-b border-slate-800 bg-slate-800/30 text-slate-400 text-xs uppercase tracking-tighter font-semibold">
                <th className="px-6 py-4">ID</th>
                <th className="px-6 py-4">Sheet Name</th>
                <th className="px-6 py-4">Last Modified</th>
                <th className="px-6 py-4">Status</th>
                <th className="px-6 py-4 text-right">Action</th>
              </tr>
            </thead>
            <tbody className="text-sm divide-y divide-slate-800/50">
              {rows.map((row) => (
                <tr
                  key={row.id}
                  className="hover:bg-slate-800/20 transition-colors group"
                >
                  <td className="px-6 py-4 font-mono text-blue-400">
                    {row.id}
                  </td>
                  <td className="px-6 py-4 text-slate-200 font-medium">
                    {row.name}
                  </td>
                  <td className="px-6 py-4 text-slate-400">{row.date}</td>
                  <td className="px-6 py-4">
                    <span
                      className={`px-2 py-1 rounded-full text-[10px] font-bold uppercase ${
                        row.status === "Active"
                          ? "bg-blue-500/10 text-blue-400"
                          : row.status === "Completed"
                            ? "bg-emerald-500/10 text-emerald-400"
                            : "bg-slate-800 text-slate-400"
                      }`}
                    >
                      {row.status}
                    </span>
                  </td>
                  <td className="px-6 py-4 text-right">
                    <button className="text-slate-500 hover:text-white transition-colors">
                      <MoreHorizontal size={18} />
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
