# todo
- [] dataloader がいまいち効いてない？のか効いてるのか...使い方が間違えているのか？sql 発行数自体は通常よりも少ないっぽい.
```
{
  getPosts {
    id
    title,
    user {
      id,
      name,
      posts {
        id,
        title,
        user {
          id,
          name,
          posts {
            id,
            title
          }
        }
      }
    }
  }
}
```