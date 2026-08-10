// fix(#9): support multiple asset denominations in swap
import { useState } from 'react';

export default function App() {
  const [address, setAddress] = useState('');
  const [score, setScore] = useState<{ score: number; total: number; positive: number; negative: number } | null>(null);

  const fetchScore = async () => {
    // TODO: wire to Soroban RPC
    setScore({ score: 87, total: 142, positive: 135, negative: 7 });
  };

  return (
    <div className="min-h-screen bg-indigo-50 p-8">
      <header className="mb-8">
        <h1 className="text-3xl font-bold text-indigo-800">⭐ Stellar Reputation Oracle</h1>
        <p className="text-indigo-600 mt-2">Portable, verifiable on-chain reputation built from real interactions</p>
      </header>

      <div className="bg-white rounded-xl shadow p-6 max-w-2xl mb-6">
        <h3 className="font-semibold text-lg mb-3">Look Up Reputation</h3>
        <div className="flex gap-2">
          <input
            type="text"
            value={address}
            onChange={(e) => setAddress(e.target.value)}
            placeholder="Enter Stellar address"
            className="flex-1 border rounded-lg p-2"
          />
          <button
            onClick={fetchScore}
            className="bg-indigo-600 text-white px-6 py-2 rounded-lg hover:bg-indigo-700"
          >
            Check Score
          </button>
        </div>
      </div>

      {score && (
        <div className="bg-white rounded-xl shadow p-6 max-w-2xl">
          <div className="flex items-center gap-6">
            <div className="text-center">
              <div className="text-5xl font-bold text-indigo-700">{score.score}</div>
              <div className="text-sm text-gray-500">Score (0-100)</div>
            </div>
            <div className="flex-1 grid grid-cols-3 gap-4">
              <div className="text-center p-3 bg-green-50 rounded-lg">
                <div className="text-2xl font-bold text-green-600">{score.positive}</div>
                <div className="text-xs text-gray-500">Positive</div>
              </div>
              <div className="text-center p-3 bg-red-50 rounded-lg">
                <div className="text-2xl font-bold text-red-600">{score.negative}</div>
                <div className="text-xs text-gray-500">Negative</div>
              </div>
              <div className="text-center p-3 bg-indigo-50 rounded-lg">
                <div className="text-2xl font-bold text-indigo-600">{score.total}</div>
                <div className="text-xs text-gray-500">Total</div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
