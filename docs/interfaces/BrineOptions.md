[@brine-db/brine](../README.md) / [Exports](../modules.md) / BrineOptions

# Interface: BrineOptions\<T\>

## Type parameters

| Name |
| :------ |
| `T` |

## Table of contents

### Properties

- [deserialize](BrineOptions.md#deserialize)
- [serialize](BrineOptions.md#serialize)

## Properties

### deserialize

• `Optional` **deserialize**: (`value`: `string`) => `T` \| `Promise`\<`T`\>

Custom deserialization function

**`Default`**

```ts
JSON.parse
```

#### Type declaration

▸ (`value`): `T` \| `Promise`\<`T`\>

##### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `value` | `string` | The value to deserialize |

##### Returns

`T` \| `Promise`\<`T`\>

#### Defined in

[index.ts:18](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L18)

___

### serialize

• `Optional` **serialize**: (`value`: `T`) => `string` \| `Promise`\<`string`\>

Custom serialization function

**`Default`**

```ts
JSON.stringify
```

#### Type declaration

▸ (`value`): `string` \| `Promise`\<`string`\>

##### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `value` | `T` | The value to serialize |

##### Returns

`string` \| `Promise`\<`string`\>

#### Defined in

[index.ts:11](https://github.com/dan-online/brinedb/blob/e69184f5117bdb7c0353fe309db857f80b55703a/src/index.ts#L11)
