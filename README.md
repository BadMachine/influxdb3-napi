<p align="center">
  <img src="https://github.com/user-attachments/assets/8a984535-1704-44af-ba85-bd6859f99949" alt="influxdb3-napi Logo" width="300"/>
</p>

# influxdb3-napi

High-performance Node.js client for InfluxDB 3.0 with native Rust bindings, supporting both read and write operations.

## Installation

```bash
npm install @badmachine/influxdb3-napi
```

## Features

- **High Performance** - Native Rust bindings for optimal performance
- **SQL Queries** - Execute SQL queries with async iterator support
- **Line Protocol Writing** - Write data using InfluxDB line protocol
- **TypeScript Support** - Full TypeScript definitions included
- **Type Safe** - Built with type safety in mind

# Why?
- This library was initially inspired by the need to handle edge cases where other libraries fail to decode certain Arrow Flight data types (see [InfluxCommunity/influxdb3-js#590](https://github.com/InfluxCommunity/influxdb3-js/issues/590)). It correctly supports all data types returned by InfluxDB.
- **Unlike this library**, some requests to `https` hosts were failing with other JS libraries due to self-signed certificate check errors
-  ~~Blazingly™~~ Much  faster than other libraries when querying the data.
- Includes three different serializers for maximum flexibility:
  - **Default serializer** — conveniently converts time intervals.
  - **Serde-based serializer** — leverages `serde` for basic json serialization.
  - **Raw serializer** — returns the raw byte array buffer.

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

## TypeScript Support

Full TypeScript definitions are included:

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
