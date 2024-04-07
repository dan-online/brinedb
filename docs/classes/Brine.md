[brinedb](../README.md) / [Exports](../modules.md) / Brine

# Class: Brine\<T\>

Brine is a simple key-value store that persists data using Rust SeaORM bindings

**`Param`**

The connection URI to the database

**`Param`**

Options for custom serialization and deserialization

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

- [connectionURI](Brine.md#connectionuri)
- [deserialize](Brine.md#deserialize)
- [initialized](Brine.md#initialized)
- [serialize](Brine.md#serialize)

### Accessors

- [internals](Brine.md#internals)

### Methods

- [clear](Brine.md#clear)
- [count](Brine.md#count)
- [delete](Brine.md#delete)
- [deleteMany](Brine.md#deletemany)
- [ensure](Brine.md#ensure)
- [get](Brine.md#get)
- [has](Brine.md#has)
- [init](Brine.md#init)
- [initCheck](Brine.md#initcheck)
- [set](Brine.md#set)

## Constructors

### constructor

• **new Brine**\<`T`\>(`connectionURI`, `options?`): [`Brine`](Brine.md)\<`T`\>

#### Type parameters

| Name | Type |
| :------ | :------ |
| `T` | `unknown` |

#### Parameters

| Name | Type |
| :------ | :------ |
| `connectionURI` | `string` |
| `options?` | [`BrineOptions`](../interfaces/BrineOptions.md)\<`T`\> |

#### Returns

[`Brine`](Brine.md)\<`T`\>

#### Defined in

[index.ts:47](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L47)

## Properties

### connectionURI

• `Private` `Readonly` **connectionURI**: `string`

#### Defined in

[index.ts:40](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L40)

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

[index.ts:44](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L44)

___

### initialized

• `Private` **initialized**: `boolean` = `false`

#### Defined in

[index.ts:42](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L42)

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

[index.ts:45](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L45)

## Accessors

### internals

• `get` **internals**(): `__module`

#### Returns

`__module`

#### Defined in

[index.ts:59](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L59)

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

[index.ts:133](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L133)

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

[index.ts:177](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L177)

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

[index.ts:148](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L148)

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

[index.ts:163](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L163)

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

[index.ts:201](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L201)

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

[index.ts:92](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L92)

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

[index.ts:221](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L221)

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

[index.ts:75](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L75)

___

### initCheck

▸ **initCheck**(): `void`

#### Returns

`void`

#### Defined in

[index.ts:227](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L227)

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

[index.ts:115](https://github.com/dan-online/brinedb/blob/2dfef7b/src/index.ts#L115)
