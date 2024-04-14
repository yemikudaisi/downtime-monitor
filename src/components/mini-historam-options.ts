import type { VueUiSparkHistogramConfig } from "vue-data-ui";

export const config: VueUiSparkHistogramConfig = {
  "style": {
    "backgroundColor": "#14171A",
    "fontFamily": "inherit",
    "animation": {
      "show": true,
      "speedMs": 500
    },
    "layout": {
      "height": 159,
      "width": 867,
      "padding": {
        "top": 0,
        "right": 0,
        "left": 0,
        "bottom": 64
      }
    },
    "bars": {
      "shape": "square",
      "strokeWidth": 0,
      "colors": {
        "gradient": {
          "show": true
        }
      },
      "borderRadius": 0,
      "gap": 12
    },
    "labels": {
      "value": {
        "fontSize": 14,
        "color": "#CCCCCC",
        "bold": false,
        "rounding": 1,
        "prefix": "",
        "suffix": "",
        "offsetY": -4
      },
      "valueLabel": {
        "fontSize": 14,
        "color": "#777777",
        "bold": false,
        "rounding": 0
      },
      "timeLabel": {
        "fontSize": 24,
        "color": "#CCCCCC",
        "bold": false
      }
    },
    "selector": {
      "stroke": "#5f8bee",
      "borderRadius": 12,
      "strokeWidth": 2,
      "strokeDasharray": 0
    },
    "title": {
      "textAlign": "left",
      "text": "12 Hours statistics",
      "color": "#FAFAFA",
      "fontSize": 14,
      "bold": false,
      "margin": "8px 0 24px 8px",
      "subtitle": {
        "color": "#A1A1A1",
        "text": "",
        "fontSize": 12,
        "bold": false
      }
    }
  }
}