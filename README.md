# Rusttery

SDK, para desarrollo sobre informacion de la batería en Windows y Linux; en Rust con interfaz CLI, GUI y API REST.

## Características

- **Monitoreo en tiempo real**: Seguimiento continuo del estado de la batería del sistema
- **Múltiples interfaces**: CLI, GUI y servidor API REST
- **Histórico de datos**: Almacenamiento SQLite de métricas de batería
- **Estadísticas avanzadas**: Análisis de consumo y tendencias
- **Multiplataforma**: Soporte para Windows y Linux
- **Sin dependencias externas**: Binario estático auto-contenido

## Información Disponible

### Linux
- Porcentaje de carga
- Estado de salud (health)
- Estado de carga (Charging/Discharging/Full)
- Ciclos de carga
- Voltaje actual
- Corriente actual
- Potencia instantánea
- Tecnología de batería
- Fabricante y modelo
- Número de serie
- Capacidad actual y de diseño
- Tiempo estimado hasta vacío/lleno

### Windows
- Porcentaje de carga
- Estado de salud (health)
- Estado de carga (Charging/Discharging/Idle)
- Potencia instantánea
- Capacidades
- Tiempo estimado hasta vacío/lleno

## Requisitos

- Rust 1.70 o superior
- Cargo

### Linux
- Acceso a `/sys/class/power_supply/`

### Windows
- Windows 10 o superior

## Instalación

```bash
git clone https://github.com/DevnisG/Rusttery.git
cd Rusttery
cargo build --release
```

El binario compilado estará disponible en `target/release/rusttery`

## Uso

### GUI (Interfaz Gráfica)

Modo por defecto. Muestra todos los datos de batería en una ventana:

```bash
cargo run
```

o

```bash
./target/release/rusttery
```

### CLI (Línea de Comandos)

Monitoreo continuo en terminal con actualización cada 3 segundos:

```bash
cargo run -- --cli
```

### API REST

Inicia un servidor HTTP en el puerto 3000:

```bash
cargo run -- --api
```

### JSON Output

Imprime los datos actuales en formato JSON y finaliza:

```bash
cargo run -- --json
```

## API REST

El servidor API se ejecuta en `http://localhost:3000` y provee los siguientes endpoints:

### Endpoints Disponibles

#### `GET /api/v1/battery/check`
Retorna porcentaje de carga y estado de salud.

**Respuesta:**
```json
{
  "status": "ok",
  "data": {
    "percent": 85,
    "health": 92
  }
}
```

#### `GET /api/v1/battery/status`
Retorna únicamente el porcentaje de carga actual.

**Respuesta:**
```json
{
  "status": "ok",
  "data": {
    "percent": 85
  }
}
```

#### `GET /api/v1/battery/health`
Retorna únicamente el estado de salud de la batería.

**Respuesta:**
```json
{
  "status": "ok",
  "data": {
    "health": 92
  }
}
```

#### `GET /api/v1/battery/full`
Retorna toda la información disponible de la batería.

**Respuesta:**
```json
{
  "status": "ok",
  "data": {
    "percent": 85,
    "health": 92,
    "status": "Discharging",
    "cycle_count": 245,
    "voltage_now": 12.6,
    "current_now": 1.8,
    "power_now": 22.68,
    "technology": "Li-ion",
    "manufacturer": "LGC",
    "model": "Battery Model",
    "serial_number": "12345",
    "capacity_full": 45000,
    "capacity_design": 50000,
    "time_to_empty": 120,
    "time_to_full": null
  }
}
```

#### `GET /api/v1/battery/history?hours=24`
Retorna el historial de mediciones de batería.

**Parámetros:**
- `hours` (opcional): Número de horas hacia atrás. Por defecto: 24

**Respuesta:**
```json
{
  "status": "ok",
  "data": {
    "hours": 24,
    "records": [
      {
        "timestamp": 1737331200,
        "percent": 85,
        "health": 92,
        "status": "Discharging",
      }
    ]
  }
}
```

#### `GET /api/v1/battery/statistics?hours=24`
Retorna estadísticas agregadas del período especificado.

**Parámetros:**
- `hours` (opcional): Número de horas hacia atrás. Por defecto: 24

**Respuesta:**
```json
{
  "status": "ok",
  "data": {
    "hours": 24,
    "statistics": {
      "avg_percent": 78.5,
      "min_percent": 45,
      "max_percent": 100,
      "avg_power": 15.3,
      "total_records": 288
    }
  }
}
```

## Base de Datos

Los datos se almacenan automáticamente en SQLite cuando se ejecuta el CLI o el servidor API.

**Ubicación:**
- Linux: `~/.local/share/rusttery/battery_history.db`
- Windows: `C:\Users\<Usuario>\AppData\Local\rusttery\battery_history.db`

**Retención:**
Los registros más antiguos de 30 días se eliminan automáticamente al iniciar la aplicación.

## Estructura del Proyecto

```
src/
├── core/         # Lógica de lectura de batería
├── api/          # Servidor HTTP REST
├── cli/          # Interfaz de línea de comandos
├── gui/          # Interfaz gráfica
├── database/     # Manejo de SQLite
└── main.rs       # Punto de entrada
```

## Desarrollo

### Compilar para Producción

```bash
cargo build --release
```

### Formatear Código

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Plataformas Soportadas

| Plataforma |    Estado    |
|------------|--------------|
| Linux      | Completo     |
| Windows    | Completo     |
| macOS      | No soportado |

## Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Fork el repositorio
2. Crea una rama para tu feature (`git checkout -b feature/nueva-funcionalidad`)
3. Commit tus cambios (`git commit -am 'Añade nueva funcionalidad'`)
4. Push a la rama (`git push origin feature/nueva-funcionalidad`)
5. Abre un Pull Request

## Licencia

MIT License - ver el archivo LICENSE para más detalles.

## Autor

**DevnisG** - [GitHub](https://github.com/Rusttery/DevnisG)
