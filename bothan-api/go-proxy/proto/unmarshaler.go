package query

import (
	"encoding/json"
	"fmt"
)

func (p *PriceOption) UnmarshalJSON(data []byte) error {
	var optionStr string
	if err := json.Unmarshal(data, &optionStr); err != nil {
		return err
	}

	switch optionStr {
	case "PRICE_OPTION_UNSPECIFIED":
		*p = PriceOption_PRICE_OPTION_UNSPECIFIED
	case "PRICE_OPTION_UNSUPPORTED":
		*p = PriceOption_PRICE_OPTION_UNSUPPORTED
	case "PRICE_OPTION_UNAVAILABLE":
		*p = PriceOption_PRICE_OPTION_UNAVAILABLE
	case "PRICE_OPTION_AVAILABLE":
		*p = PriceOption_PRICE_OPTION_AVAILABLE
	default:
		return fmt.Errorf("unknown PriceOption value: %s", optionStr)
	}

	return nil
}
