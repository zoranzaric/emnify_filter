## Example

event\_type 6

```json
{
  "id": 201370709,
  "alert": false,
  "description": "New location received from VLR for IMSI='90143012345678912345', now attached to VLR='491720013095'.",
  "timestamp": "2017-10-26T07:28:00.000+0000",
  "event_type": {
    "id": 1,
    "description": "Update location"
  },
  "event_source": {
    "id": 0,
    "description": "Network"
  },
  "event_severity": {
    "id": 0,
    "description": "Info"
  },
  "organisation": {
    "id": 839921,
    "name": "Demo Company"
  },
  "endpoint": {
    "id": 8638726,
    "name": "GPS Tracker",
    "ip_address": "100.96.234.249",
    "tags": null,
    "imei": "3577620833012201"
  },
  "imsi": {
    "id": 205672,
    "imsi": "90143012345678912345",
    "import_date": "2016-12-27T10:09:23.000+0000"
  },
  "sim": {
    "id": 274887,
    "iccid": "8988303001234567890",
    "production_date": "2016-12-27T10:09:23.000+0000"
  },
  "detail": {
    "id": 3,
    "name": "Vodafone",
    "country": {
      "id": 74,
      "name": "Germany",
      "country_code": "49",
      "mcc": "262",
      "iso_code": "de"
    },
    "tapcode": [{
      "id": 2,
      "tapcode": "DEUD2"
    }],
    "mnc": [{
      "id": 3,
      "mnc": "02"
    }]
  }
}
```
