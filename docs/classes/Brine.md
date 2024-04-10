[@brine-db/brine](../README.md) / [Exports](../modules.md) / Brine

# Class: Brine\<T\>

Brine is a simple key-value store that persists data using Rust SeaORM bindings

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
- [deserialize](Brine.md#deserialize)
- [serialize](Brine.md#serialize)

### Methods

- [clear](Brine.md#clear)
- [close](Brine.md#close)
- [count](Brine.md#count)
- [delete](Brine.md#delete)
- [deleteMany](Brine.md#deletemany)
- [ensure](Brine.md#ensure)
- [get](Brine.md#get)
- [has](Brine.md#has)
- [init](Brine.md#init)
- [set](Brine.md#set)
- [setMany](Brine.md#setmany)

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

[index.ts:48](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L48)

## Properties

### db

• `Private` **db**: `BrineDb`

#### Defined in

[index.ts:37](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L37)

___

### deserialize

• `Private` **deserialize**: (`value`: `string`) => `T` \| `Promise`\<`T`\> = `JSON.parse`

#### Type declaration

▸ (`value`): `T` \| `Promise`\<`T`\>

##### Parameters

| Name | Type |
| :------ | :------ |
| `value` | `string` |

##### Returns

`T` \| `Promise`\<`T`\>

#### Defined in

[index.ts:39](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L39)

___

### serialize

• `Private` **serialize**: (`value`: `T`) => `string` \| `Promise`\<`string`\> = `JSON.stringify`

#### Type declaration

▸ (`value`): `string` \| `Promise`\<`string`\>

##### Parameters

| Name | Type |
| :------ | :------ |
| `value` | `T` |

##### Returns

`string` \| `Promise`\<`string`\>

#### Defined in

[index.ts:40](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L40)

## Methods

### clear

▸ **clear**(): `Promise`\<`void`\>

Clear all values from the database

#### Returns

`Promise`\<`void`\>

Promise<void>

**`Example`**

```ts
await brinedb.clear();
```

#### Defined in

[index.ts:125](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L125)

___

### close

▸ **close**(): `Promise`\<`void`\>

Close the connection to the database

#### Returns

`Promise`\<`void`\>

Promise<void>

**`Example`**

```ts
await brinedb.close();
```

#### Defined in

[index.ts:239](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L239)

___

### count

▸ **count**(): `Promise`\<`number`\>

Count all documents in the database

#### Returns

`Promise`\<`number`\>

Promise<number>

**`Example`**

```ts
const count = await brinedb.count();
```

#### Defined in

[index.ts:169](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L169)

___

### delete

▸ **delete**(`key`): `Promise`\<`void`\>

Delete a key from the database

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `key` | `string` | The key to delete |

#### Returns

`Promise`\<`void`\>

Promise<void>

**`Example`**

```ts
await brinedb.delete("key");
```

#### Defined in

[index.ts:140](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L140)

___

### deleteMany

▸ **deleteMany**(`keys`): `Promise`\<`void`\>

Delete multiple keys from the database

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `keys` | `string`[] | The keys to delete |

#### Returns

`Promise`\<`void`\>

Promise<void>

**`Example`**

```ts
await brinedb.deleteMany(["key1", "key2"]);
```

#### Defined in

[index.ts:155](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L155)

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

[index.ts:191](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L191)

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

[index.ts:86](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L86)

___

### has

▸ **has**(`key`): `Promise`\<`boolean`\>

Check if a key exists in the database

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `key` | `string` | The key to check |

#### Returns

`Promise`\<`boolean`\>

Promise<boolean>

**`Example`**

```ts
const exists = await brinedb.has("key");
```

#### Defined in

[index.ts:211](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L211)

___

### init

▸ **init**(): `Promise`\<`void`\>

Initialize the Brine instance and run migrations

#### Returns

`Promise`\<`void`\>

Promise<void>

**`Example`**

```ts
await brinedb.init();
```

#### Defined in

[index.ts:70](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L70)

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

[index.ts:107](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L107)

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

#### Defined in

[index.ts:221](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L221)
