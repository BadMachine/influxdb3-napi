<p align="center">
  <img src="https://github.com/user-attachments/assets/8a984535-1704-44af-ba85-bd6859f99949" alt="influxdb3-napi Logo" width="300"/>
</p>

# influxdb3-napi

High-performance Node.js client for InfluxDB 3.0 with native Rust bindings, supporting both read and write operations.

## Features

- **High Performance** - Native Rust bindings for optimal performance
- **SQL Queries** - Execute SQL queries with async iterator support
- **Line Protocol Writing** - Write data using InfluxDB line protocol
- **TypeScript Support** - Full TypeScript definitions included
- **Type Safe** - Built with type safety in mind

## Installation

```bash
npm install @badmachine/influxdb3-napi
```

## Quick Start

```javascript
import { InfluxDbClient, Point } from '@badmachine/influxdb3-napi';

// Initialize client
const client = new InfluxDbClient(
  'http://your-influxdb-host:8086',
  'your-api-token'
);

// Write data using Point builder
const point = Point.fromMeasurement('temperature')
  .setTag('location', 'office')
  .setTag('sensor', 'temp01')
  .setBooleanField('active', true)
  .setFloatField('value', 23.5);

const lineProtocol = point.toLineProtocol('ns');
await client.write([lineProtocol], 'your-database');

// Query data with async iteration
const result = client.query({
  database: 'your-database',
  query: 'SELECT * FROM temperature WHERE time > now() - 1h',
  type: 'sql'
});

// Stream results efficiently
for await (const row of result) {
  console.log(row);
}
```

## API Reference

### InfluxDbClient

#### Constructor

```javascript
new InfluxDbClient(host, token)
```

**Parameters:**
- `host` (string) - InfluxDB server URL (e.g., 'http://localhost:8086')
- `token` (string) - Authentication token

#### Methods

##### `query(options)`

Execute a SQL query and return an async iterator.

**Parameters:**
- `options` (object):
  - `database` (string) - Target database name
  - `query` (string) - SQL query string
  - `type?` (string) - Query type, defaults to 'sql'

**Returns:** `AsyncIterator<object>` - Async iterator over query results

**Example:**
```javascript
const queryResult = client.query({
  database: 'mydb',
  query: 'SELECT mean(temperature) FROM sensors WHERE time > now() - 1h GROUP BY time(10m)',
  type: 'sql'
});

const results = [];
for await (const row of queryResult) {
  results.push(row);
}
```

##### `write(lineProtocols, database, options?)`

Write data using line protocol format.

**Parameters:**
- `lineProtocols` (string[]) - Array of line protocol strings
- `database` (string) - Target database name
- `options?` (object):
  - `noSync?` (boolean) - Disable synchronous write (default: false)
  - `precision?` (object) - Timestamp precision
    - `type` ('V2') - Precision type
    - `field0` ('ns'|'us'|'ms'|'s') - Time unit
  - `gzip?` (boolean) - Enable gzip compression (default: false)

**Example:**
```javascript
const lineProtocols = [
  'temperature,location=office value=23.5 1234567890000000000',
  'humidity,location=office value=45.2 1234567890000000000'
];

await client.write(lineProtocols, 'sensors', {
  noSync: false,
  precision: { type: 'V2', field0: 'ns' },
  gzip: false
});
```

### Point Builder

#### `Point.fromMeasurement(measurement)`

Create a new Point instance.

**Parameters:**
- `measurement` (string) - Measurement name

**Returns:** Point instance for method chaining

#### Point Methods

##### `setTag(key, value)`
Set a tag key-value pair.

##### `setBooleanField(key, value)`
Set a boolean field.

##### `setFloatField(key, value)`
Set a numeric field.

##### `setStringField(key, value)`
Set a string field.

##### `toLineProtocol(precision?)`
Convert point to line protocol string.

**Example:**
```javascript
const point = Point.fromMeasurement('cpu_usage')
  .setTag('host', 'server01')
  .setTag('region', 'us-east-1')
  .setFloatField('usage_percent', 85.2)
  .setBooleanField('alert', false);

const lineProtocol = point.toLineProtocol('ns');
// Result: cpu_usage,host=server01,region=us-east-1 usage_percent=85.2,alert=false
```


## TypeScript Support

Full TypeScript definitions are included:

```typescript
import { InfluxDbClient, Point } from '@badmachine/influxdb3-napi';

interface SensorData {
  temperature: number;
  humidity: number;
  timestamp: number;
}

const client = new InfluxDbClient(
  process.env.INFLUX_HOST!,
  process.env.INFLUX_TOKEN!
);

const point = Point.fromMeasurement('sensors')
  .setTag('location', 'office')
  .setFloatField('temperature', 23.5)
  .setFloatField('humidity', 45.2);

await client.write([point.toLineProtocol('ns')], 'environmental');
```

## Error Handling

```javascript
try {
  const result = client.query({
    database: 'mydb',
    query: 'SELECT * FROM measurements'
  });
  
  for await (const row of result) {
    console.log(row);
  }
} catch (error) {
  console.error('Query failed:', error.message);
}

try {
  await client.write(['invalid line protocol'], 'mydb');
} catch (error) {
  console.error('Write failed:', error.message);
}
```


## Contributing

Contributions are welcome! Please read our contributing guidelines and submit pull requests to our repository.

## License

MIT License - see LICENSE file for details.

## Support

- [Documentation](https://github.com/badmachine/influxdb3-napi)
- [Issue Tracker](https://github.com/badmachine/influxdb3-napi/issues)
- [Discussions](https://github.com/badmachine/influxdb3-napi/discussions)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and changes.
