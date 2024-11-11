import { createSignal } from 'solid-js';
import { fetchWithAuth } from '@/utils/fetch';
 // Update the path based on your project structure

function TestFetchButton() {
  const [result, setResult] = createSignal<string | null>(null);

  const handleFetch = async () => {
    try {
      const response = await fetchWithAuth({
        path: '/api/getcredentials',
        method: 'GET',
      });
      setResult(JSON.stringify(response));
    } catch (error) {
      setResult('Error occurred');
      console.error('Fetch error:', error);
    }
  };

  return (
    <div>
      <button
        onClick={handleFetch}
        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
      >
        Test Fetch with Auth
      </button>
      {result() && (
        <pre class="mt-4 p-2 border border-gray-300 rounded bg-gray-50">
          {result()}
        </pre>
      )}
    </div>
  );
};

export default TestFetchButton;
