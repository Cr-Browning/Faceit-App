import { invoke } from '@backend/rust_fw/src/api_calls';

async function fetchPlayerStats(playerName) {
  const stats = await invoke('get_faceit_player_stats', { playerName });
  console.log(stats);
}