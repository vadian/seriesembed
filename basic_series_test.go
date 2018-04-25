package emseries_test

import (
	. "github.com/luminescent-dreams/emseries"

	"github.com/onsi/ginkgo"
	"github.com/onsi/gomega"
	"strconv"
	"time"
)

func Float64(v float64) *float64              { return &v }
func Duration(v time.Duration) *time.Duration { return &v }
func String(v string) *string                 { return &v }

type BikeRide struct {
	date     time.Time
	distance *float64
	duration *time.Duration
	notes    *string
	gpsPath  *string
}

func (self *BikeRide) Timestamp() time.Time { return self.date }
func (self *BikeRide) Tags() []string       { return []string{} }
func (self *BikeRide) Values() []string {
	return []string{
		strconv.FormatFloat(*self.distance, 'E', -1, 64),
		self.duration.String(),
	}
}

var _ = ginkgo.Describe("BasicSeries", func() {
	ginkgo.It("records and retrieves records based on exact time", func() {
		tz, err := time.LoadLocation("US/Eastern")
		gomega.Expect(err).To(gomega.BeNil())

		series, err := NewSeries("var/series1")
		gomega.Expect(err).To(gomega.BeNil())

		rideTime := time.Date(2018, 2, 1, 12, 0, 0, 0, tz)
		var ride1 BikeRide
		{
			ride1 = BikeRide{
				rideTime,
				Float64(5.5),
				Duration(time.Duration(20*60) * time.Second),
				String("first demo ride"),
				nil}
		}

		err = series.Put(&ride1)
		gomega.Expect(err).To(gomega.BeNil())

		criteria := And{Start_(rideTime, true), End_(rideTime, true)}
		records, err := series.Search(&criteria)
		gomega.Expect(err).To(gomega.BeNil())
		gomega.Expect(len(records)).To(gomega.Equal(1))
		gomega.Expect(records[0]).To(gomega.Equal(ride1))
	})

	ginkgo.PIt("returns records in order", func() {
	})

	ginkgo.PIt("filters records based on a time range", func() {
	})

	ginkgo.PIt("filters records based on tags", func() {
	})

	ginkgo.PIt("filters records based on both time and tags", func() {
	})
})
