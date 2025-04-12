
5. `MONITORING_OPERATIONS.md`:
```markdown
# Monitoring and Operations

## 1. System Metrics

### 1.1 Collection Metrics
- Block processing rate
- Transaction processing rate
- Event streaming latency
- Error rates

### 1.2 Storage Metrics
- ClickHouse query performance
- Cache hit/miss ratio
- Storage growth rate
- Query latency

### 1.3 Processing Metrics
- Balance calculation time
- Stream processing lag
- State update latency
- Error rates

## 2. Alerting

### 2.1 Critical Alerts
- Data collection stops
- High error rates
- Processing delays
- API failures

### 2.2 Warning Alerts
- Cache miss ratio high
- Processing latency increase
- Resource utilization high
- Rate limiting triggers

## 3. Logging

### 3.1 Log Categories
- Application logs
- Access logs
- Error logs
- Audit logs

### 3.2 Log Format
```json
{
    "timestamp": "ISO8601",
    "level": "INFO|WARN|ERROR",
    "service": "service_name",
    "message": "log message",
    "metadata": {
        "chain_id": "1",
        "block_number": "15000000"
    }
}
```
```

Would you like me to create any additional documentation or elaborate on any specific section?