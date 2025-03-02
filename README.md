# Time Predict

### Instalação
```sh
cargo install --git https://github.com/kesleymartins/time-predict
```

### Exeplos
- Comando
```sh
time-predict 8:00 11:00 12:00
```

- Saida
```sh
==== Logs ====
> 08:00
< 11:00
> 12:00

==== Logs ====
saida prevista: 17:48
saida real: -
saldo: -05:48
```

- Comando
```sh
time-predict 8:00 11:00 12:00 18:00
```

- Saida
```sh
==== Logs ====
> 08:00
< 11:00
> 12:00
< 18:00

==== Logs ====
saida prevista: -
saida real: 18:00
saldo: +00:12
```
