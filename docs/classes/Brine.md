[@brine-db/brine](../README.md) / [Exports](../modules.md) / Brine

# Class: Brine\<T\>

Brine is a simple key-value store that persists data using Rust bindings

**`Example`**

```ts
import { Brine } from "brinedb";

const brine = new Brine("sqlite::memory:");

await brine.init();

await brine.set("key", "value");
const value = await brine.get("key");
```

## Type parameters

| Name | Type |
| :------ | :------ |
| `T` | `unknown` |

## Table of contents

### Constructors

- [constructor](Brine.md#constructor)

### Properties

- [db](Brine.md#db)
- [internalDeserialize](Brine.md#internaldeserialize)
- [internalSerialize](Brine.md#internalserialize)

### Methods

- [clear](Brine.md#clear)
- [close](Brine.md#close)
- [count](Brine.md#count)
- [delete](Brine.md#delete)
- [deleteMany](Brine.md#deletemany)
- [deserialize](Brine.md#deserialize)
- [ensure](Brine.md#ensure)
- [get](Brine.md#get)
- [getMany](Brine.md#getmany)
- [has](Brine.md#has)
- [init](Brine.md#init)
- [keys](Brine.md#keys)
- [serialize](Brine.md#serialize)
- [set](Brine.md#set)
- [setMany](Brine.md#setmany)
- [values](Brine.md#values)

## Constructors

### constructor

• **new Brine**\<`T`\>(`connectionURI`, `options?`): [`Brine`](Brine.md)\<`T`\>

Create a new Brine instance

#### Type parameters

| Name | Type |
| :------ | :------ |
| `T` | `unknown` |

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `connectionURI` | `string` | The connection URI to the database |
| `options?` | [`BrineOptions`](../interfaces/BrineOptions.md)\<`T`\> | Options for custom serialization and deserialization |

#### Returns

[`Brine`](Brine.md)\<`T`\>

#### Defined in

[index.ts:49](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L49)

## Properties

### db

• `Private` **db**: `BrineDb`

#### Defined in

[index.ts:37](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L37)

___

### internalDeserialize

• `Private` **internalDeserialize**: (`value`: `string`) => `T` \| `Promise`\<`T`\> = `JSON.parse`

#### Type declaration

▸ (`value`): `T` \| `Promise`\<`T`\>

##### Parameters

| Name | Type |
| :------ | :------ |
| `value` | `string` |

##### Returns

`T` \| `Promise`\<`T`\>

#### Defined in

[index.ts:39](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L39)

___

### internalSerialize

• `Private` **internalSerialize**: (`value`: `T`) => `string` \| `Promise`\<`string`\> = `JSON.stringify`

#### Type declaration

▸ (`value`): `string` \| `Promise`\<`string`\>

##### Parameters

| Name | Type |
| :------ | :------ |
| `value` | `T` |

##### Returns

`string` \| `Promise`\<`string`\>

#### Defined in

[index.ts:40](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L40)

## Methods

### clear

▸ **clear**(): `void`

Clear all values from the database

#### Returns

`void`

Promise<void>

**`Example`**

```ts
await brinedb.clear();
```

#### Defined in

[index.ts:126](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L126)

___

### close

▸ **close**(): `void`

Close the connection to the database

#### Returns

`void`

Promise<void>

**`Example`**

```ts
await brinedb.close();
```

#### Defined in

[index.ts:296](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L296)

___

### count

▸ **count**(): `number`

Count all documents in the database

#### Returns

`number`

Promise<number>

**`Example`**

```ts
const count = await brinedb.count();
```

#### Defined in

[index.ts:170](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L170)

___

### delete

▸ **delete**(`key`): `void`

Delete a key from the database

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `key` | `string` | The key to delete |

#### Returns

`void`

Promise<void>

**`Example`**

```ts
await brinedb.delete("key");
```

#### Defined in

[index.ts:141](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L141)

___

### deleteMany

▸ **deleteMany**(`keys`): `void`

Delete multiple keys from the database

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `keys` | `string`[] | The keys to delete |

#### Returns

`void`

Promise<void>

**`Example`**

```ts
await brinedb.deleteMany(["key1", "key2"]);
```

#### Defined in

[index.ts:156](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L156)

___

### deserialize

▸ **deserialize**(`value`): `Promise`\<`T`\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `value` | `string` |

#### Returns

`Promise`\<`T`\>

#### Defined in

[index.ts:304](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L304)

___

### ensure

▸ **ensure**(`key`, `value`): `Promise`\<`T`\>

Ensure a key exists in the database

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `key` | `string` | The key to check |
| `value` | `T` | The value to set if the key doesn't exist |

#### Returns

`Promise`\<`T`\>

The value that was set or the existing value

**`Example`**

```ts
const value = await brinedb.ensure("key", "value");

value === "value"; // true

const changed = await brinedb.ensure("key", "changed");

changed === "value"; // true
```

#### Defined in

[index.ts:192](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L192)

___

### get

▸ **get**(`key`): `Promise`\<``null`` \| `T`\>

Get a value from the store

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `key` | `string` | The key to get |

#### Returns

`Promise`\<``null`` \| `T`\>

The value or null if it doesn't exist

**`Example`**

```ts
const value = await brinedb.get("key");
```

#### Defined in

[index.ts:87](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L87)

___

### getMany

▸ **getMany**(`keys`): `Promise`\<`Record`\<`string`, ``null`` \| `T`\>\>

Get many keys from the database

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `keys` | `string`[] | The keys to get |

#### Returns

`Promise`\<`Record`\<`string`, ``null`` \| `T`\>\>

Promise<[string, string][]>

**`Example`**

```ts
const data = await brinedb.getMany(["key1", "key2"]);
```

#### Defined in

[index.ts:246](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L246)

___

### has

▸ **has**(`key`): `boolean`

Check if a key exists in the database

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `key` | `string` | The key to check |

#### Returns

`boolean`

Promise<boolean>

**`Example`**

```ts
const exists = await brinedb.has("key");
```

#### Defined in

[index.ts:212](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L212)

___

### init

▸ **init**(): `void`

Initialize the Brine instance and run migrations

#### Returns

`void`

Promise<void>

**`Example`**

```ts
await brinedb.init();
```

#### Defined in

[index.ts:71](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L71)

___

### keys

▸ **keys**(): `string`[]

Get all keys from the database

#### Returns

`string`[]

Promise<string[]>

**`Example`**

```ts
const keys = await brinedb.keys();
```

#### Defined in

[index.ts:267](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L267)

___

### serialize

▸ **serialize**(`value`): `Promise`\<`string`\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `value` | `T` |

#### Returns

`Promise`\<`string`\>

#### Defined in

[index.ts:300](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L300)

___

### set

▸ **set**(`key`, `value`): `Promise`\<`T`\>

Set a value in the store

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `key` | `string` | The key to set |
| `value` | `T` | The value to set |

#### Returns

`Promise`\<`T`\>

The value that was set

**`Example`**

```ts
await brinedb.set("key", { foo: "bar" });
await brinedb.set("key", "value");
```

#### Defined in

[index.ts:108](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L108)

___

### setMany

▸ **setMany**(`data`): `Promise`\<`void`\>

Set many keys in the database

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `data` | [`string`, `T`][] | An array of 2d arrays containing keys and values |

#### Returns

`Promise`\<`void`\>

Promise<void>

**`Example`**

```ts
await brinedb.setMany([
 ["key1", "value1"],
 ["key2", "value2"],
]);

#### Defined in

[index.ts:228](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L228)

___

### values

▸ **values**(): `Promise`\<`Awaited`\<`T`\>[]\>

Get all values from the database

#### Returns

`Promise`\<`Awaited`\<`T`\>[]\>

Promise<T[]>

**`Example`**

```ts
const values = await brinedb.values();
```

#### Defined in

[index.ts:280](https://github.com/dan-online/brinedb/blob/6efaf4baf13e376fc16b5028d4379776aaa55325/src/index.ts#L280)
