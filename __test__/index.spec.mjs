import test from 'ava';

import { parseToJsonStr } from '../index.js';

test('parseToJsonStr', (t) => {
  t.is(
    parseToJsonStr('linear-gradient(135deg, #6a11cb 0%, #2575fc 100%)'),
    '{"type":"linear","vendorPrefix":[],"direction":{"type":"angle","value":{"type":"deg","value":135.0}},"items":[{"type":"color-stop","color":{"type":"rgb","r":106.0,"g":17.0,"b":203.0,"alpha":1.0},"position":{"type":"percentage","value":0.0}},{"type":"color-stop","color":{"type":"rgb","r":37.0,"g":117.0,"b":252.0,"alpha":1.0},"position":{"type":"percentage","value":1.0}}]}',
  );
});
