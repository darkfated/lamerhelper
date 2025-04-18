﻿using System;
using System.Net.Http;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Media.Imaging;
using Microsoft.Win32;

namespace LamerHelper.Modules.Tools
{
    public partial class QrCreatorModule : UserControl, IModule
    {
        public QrCreatorModule()
        {
            InitializeComponent();
        }

        public string ModuleName => "QrCreatorModule";
        public string DisplayName => "Создание QR-кода из любого текста";
        public string Category => "Инструменты";
        public string Description => "Позволяет создать QR-код из любого текста с возможностью выбора размера.";
        public UserControl GetModuleControl() => this;

        private async void ButtonGenerate_Click(object sender, RoutedEventArgs e)
        {
            string inputText = TextBoxInput.Text;
            if (string.IsNullOrWhiteSpace(inputText))
            {
                MessageBox.Show("Введите текст или ссылку!", "Ошибка", MessageBoxButton.OK, MessageBoxImage.Warning);
                return;
            }

            // Получение выбранного размера
            if (ComboBoxSize.SelectedItem is ComboBoxItem selectedSizeItem &&
                int.TryParse(selectedSizeItem.Content.ToString(), out int size))
            {
                string apiUrl = $"https://api.qrserver.com/v1/create-qr-code/?size={size}x{size}&data={Uri.EscapeDataString(inputText)}";

                using var client = new HttpClient();
                try
                {
                    HttpResponseMessage response = await client.GetAsync(apiUrl);

                    if (response.IsSuccessStatusCode)
                    {
                        byte[] imageBytes = await response.Content.ReadAsByteArrayAsync();

                        var bitmapImage = new BitmapImage();
                        bitmapImage.BeginInit();
                        bitmapImage.StreamSource = new System.IO.MemoryStream(imageBytes);
                        bitmapImage.CacheOption = BitmapCacheOption.OnLoad;
                        bitmapImage.EndInit();

                        ImageQr.Source = bitmapImage;
                        ImageQr.Width = size;
                        ImageQr.Height = size;

                        ImageQr.Visibility = Visibility.Visible;
                        PanelButtons.Visibility = Visibility.Visible;

                        MessageBox.Show("QR-код успешно создан!", "Успех", MessageBoxButton.OK, MessageBoxImage.Information);
                    }
                    else
                    {
                        MessageBox.Show("Ошибка при создании QR-кода: " + response.StatusCode, "Ошибка", MessageBoxButton.OK, MessageBoxImage.Error);
                    }
                }
                catch (Exception ex)
                {
                    MessageBox.Show("Произошла ошибка: " + ex.Message, "Ошибка", MessageBoxButton.OK, MessageBoxImage.Error);
                }
            }
            else
            {
                MessageBox.Show("Выберите корректный размер QR-кода.", "Ошибка", MessageBoxButton.OK, MessageBoxImage.Warning);
            }
        }

        private void ButtonSave_Click(object sender, RoutedEventArgs e)
        {
            if (ImageQr.Source == null) return;

            SaveFileDialog saveDialog = new SaveFileDialog
            {
                Filter = "PNG Image|*.png",
                FileName = "qrcode.png"
            };

            if (saveDialog.ShowDialog() != true) return;

            var encoder = new PngBitmapEncoder();
            encoder.Frames.Add(BitmapFrame.Create((BitmapSource)ImageQr.Source));

            using (var stream = saveDialog.OpenFile())
            {
                encoder.Save(stream);
            }

            MessageBox.Show("QR-код сохранен!", "Успех", MessageBoxButton.OK, MessageBoxImage.Information);
        }

        private void ButtonCopy_Click(object sender, RoutedEventArgs e)
        {
            if (ImageQr.Source == null) return;

            Clipboard.SetImage((BitmapSource)ImageQr.Source);
            MessageBox.Show("QR-код скопирован в буфер обмена!", "Успех", MessageBoxButton.OK, MessageBoxImage.Information);
        }
    }
}
