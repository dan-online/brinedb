[@brinedb/brine](README.md) / Exports

# @brinedb/brine

## Table of contents

### References

- [default](modules.md#default)

### Classes

- [Brine](classes/Brine.md)

### Interfaces

- [BrineOptions](interfaces/BrineOptions.md)

### Variables

- [BrineDatabases](modules.md#brinedatabases)

## References

### default

Renames and re-exports [Brine](classes/Brine.md)

## Variables

### BrineDatabases

• `Const` **BrineDatabases**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `mysql` | \{ `build`: (`options`: \{ `database`: `string` ; `host?`: `string` ; `password`: `string` ; `port?`: `number` ; `user`: `string`  }) => `string`  } |
| `mysql.build` | (`options`: \{ `database`: `string` ; `host?`: `string` ; `password`: `string` ; `port?`: `number` ; `user`: `string`  }) => `string` |
| `postgres` | \{ `build`: (`options`: \{ `database`: `string` ; `host?`: `string` ; `password`: `string` ; `port?`: `number` ; `user`: `string`  }) => `string`  } |
| `postgres.build` | (`options`: \{ `database`: `string` ; `host?`: `string` ; `password`: `string` ; `port?`: `number` ; `user`: `string`  }) => `string` |
| `sqlite` | \{ `file`: (`path`: `string`) => `string` ; `memory`: `string` = "sqlite::memory:" } |
| `sqlite.file` | (`path`: `string`) => `string` |
| `sqlite.memory` | `string` |

#### Defined in

[utils.ts:1](https://github.com/dan-online/brinedb/blob/6c3396da21325bef297b5532fc2d6f594f1ff6ce/src/utils.ts#L1)
