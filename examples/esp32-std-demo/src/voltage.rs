use core::borrow::Borrow;
use esp_idf_svc::hal::adc::oneshot::{AdcChannelDriver, AdcDriver};
use esp_idf_svc::hal::adc::{ADCCH6, ADCU1};
use esp_idf_svc::hal::delay;
use esp_idf_svc::hal::gpio::{Input, PinDriver};
use esp_idf_svc::hal::task::notification::Notification;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Padding};
use std::error::Error;
use std::marker::PhantomData;
use tui_big_text::{BigText, PixelSize};

pub struct VoltageApp<B: Backend> {
    _marker: PhantomData<B>,
}

impl<B: Backend> VoltageApp<B> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn run<'a, M>(
        mut self,
        terminal: &mut Terminal<B>,
        notification: &mut Notification,
        button: &mut PinDriver<'_, Input>,
        adc_channel: &mut AdcChannelDriver<'a, ADCCH6<ADCU1>, M>,
    ) -> Result<(), Box<dyn Error>>
    where
        B::Error: 'static,
        M: Borrow<AdcDriver<'a, ADCU1>>,
    {
        button.enable_interrupt().unwrap();
        loop {
            if notification.wait(delay::NON_BLOCK).is_some() {
                return Ok(());
            }
            if let Ok(voltage) = adc_channel.read() {
                terminal.draw(|frame| self.draw(frame, 2 * voltage))?;
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame, voltage: u16) {
        let [content_area, footer_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(frame.area());

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::vertical(1))
            .title("Battery voltage")
            .border_style(Style::new().yellow());
        let inner_area = block.inner(content_area);
        frame.render_widget(block, content_area);

        let voltage_text = format!("{:.2}V", voltage as f32 / 1000.0);
        let big_text = BigText::builder()
            .pixel_size(PixelSize::Full)
            .style(Style::new().blue())
            .lines(vec![voltage_text.into()])
            .build();
        frame.render_widget(big_text, inner_area);

        let footer = Line::raw("[S1] to change screen").centered().gray();
        frame.render_widget(footer, footer_area);
    }
}
